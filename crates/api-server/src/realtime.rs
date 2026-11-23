use actix_web::{web, Error, HttpRequest, HttpResponse};
use dashmap::DashMap;
use futures_util::StreamExt;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use sqlx::PgPool;
use std::time::Duration;
use tokio::sync::{broadcast, mpsc};
use tokio::time::interval;
use tracing::{debug, error, info, warn};
pub use umuryango_core_domain::RealtimeEvent;
use uuid::Uuid;

use crate::middleware::JwtClaims;
use crate::state::AppState;

pub struct FamilyHub {
    channels: DashMap<Uuid, broadcast::Sender<RealtimeEvent>>,
}

impl Default for FamilyHub {
    fn default() -> Self {
        Self::new()
    }
}

impl FamilyHub {
    pub fn new() -> Self {
        Self {
            channels: DashMap::new(),
        }
    }

    pub fn subscribe(&self, family_id: Uuid) -> broadcast::Receiver<RealtimeEvent> {
        let entry = self.channels.entry(family_id).or_insert_with(|| {
            let (tx, _rx) = broadcast::channel(256);
            tx
        });
        entry.subscribe()
    }

    pub fn broadcast(&self, family_id: Uuid, event: RealtimeEvent) {
        if let Some(entry) = self.channels.get(&family_id) {
            let _ = entry.send(event);
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct WsQuery {
    pub token: Option<String>,
}

async fn get_user_families(pool: &PgPool, user_id: Uuid) -> Vec<Uuid> {
    sqlx::query_scalar::<_, Uuid>(
        r#"
        SELECT family_id
        FROM family_members
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default()
}

pub async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<AppState>,
    query: web::Query<WsQuery>,
) -> Result<HttpResponse, Error> {
    // 1. Authenticate via query param or Authorization header
    let token_str = query.token.clone().or_else(|| {
        req.headers()
            .get(actix_web::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .map(|s| s.trim().to_string())
    });

    let Some(token) = token_str else {
        return Ok(HttpResponse::Unauthorized().finish());
    };

    let key = DecodingKey::from_secret(data.jwt_secret.as_bytes());
    let validation = Validation::default();
    let token_data = match decode::<JwtClaims>(&token, &key, &validation) {
        Ok(data) => data,
        Err(_) => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let user_id = token_data.claims.sub;
    info!("WebSocket client authenticating: user_id={}", user_id);

    // 2. Fetch all families the user belongs to
    let family_ids = get_user_families(&data.db, user_id).await;
    info!(
        "Subscribing user_id={} to families: {:?}",
        user_id, family_ids
    );

    // 3. Initiate WebSocket session
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?;

    // 4. Multiplex all family channels into an mpsc queue
    let (event_tx, mut event_rx) = mpsc::unbounded_channel::<RealtimeEvent>();

    for fid in family_ids {
        let mut rx = data.family_hub.subscribe(fid);
        let tx = event_tx.clone();
        actix_web::rt::spawn(async move {
            while let Ok(event) = rx.recv().await {
                if tx.send(event).is_err() {
                    break;
                }
            }
        });
    }

    // 5. Run connection event loop with keepalive heartbeat
    actix_web::rt::spawn(async move {
        let mut heartbeat = interval(Duration::from_secs(15));

        loop {
            tokio::select! {
                // Heartbeat ping
                _ = heartbeat.tick() => {
                    if session.ping(b"heartbeat").await.is_err() {
                        debug!("WebSocket heartbeat failed, terminating session for user={}", user_id);
                        break;
                    }
                }

                // Incoming broadcast event from family hub
                Some(event) = event_rx.recv() => {
                    match serde_json::to_string(&event) {
                        Ok(json) => {
                            if session.text(json).await.is_err() {
                                warn!("Failed to deliver event to user={}", user_id);
                                break;
                            }
                        }
                        Err(e) => {
                            error!("Failed to serialize RealtimeEvent: {:?}", e);
                        }
                    }
                }

                // Incoming WebSocket frames from client
                msg = msg_stream.next() => {
                    match msg {
                        Some(Ok(actix_ws::Message::Ping(bytes))) => {
                            if session.pong(&bytes).await.is_err() {
                                break;
                            }
                        }
                        Some(Ok(actix_ws::Message::Pong(_))) => {
                            // Client replied to heartbeat
                        }
                        Some(Ok(actix_ws::Message::Close(reason))) => {
                            let _ = session.clone().close(reason).await;
                            break;
                        }
                        Some(Ok(actix_ws::Message::Text(_))) => {
                            // Client messages (can be extended for bidirectional interactions)
                        }
                        Some(Ok(actix_ws::Message::Binary(_))) => {}
                        Some(Ok(actix_ws::Message::Nop)) | Some(Ok(actix_ws::Message::Continuation(_))) => {}
                        Some(Err(e)) => {
                            warn!("WebSocket stream error for user={}: {:?}", user_id, e);
                            break;
                        }
                        None => break,
                    }
                }
            }
        }

        let _ = session.close(None).await;
        debug!("WebSocket connection closed for user={}", user_id);
    });

    Ok(response)
}
