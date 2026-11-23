use actix_web::{Error, HttpRequest, HttpResponse, web};
use dashmap::DashMap;
use futures_util::StreamExt;
use jsonwebtoken::{DecodingKey, Validation, decode};
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

    pub fn broadcast_all(&self, event: RealtimeEvent) {
        for entry in self.channels.iter() {
            let _ = entry.value().send(event.clone());
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

    // 3. Initiate Actix-web WebSocket session
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?;

    // Channel for forwarding broadcasted events to this client's writer
    let (event_tx, mut event_rx) = mpsc::channel::<RealtimeEvent>(128);

    // Subscribe to each family's broadcast channel
    for fid in family_ids {
        let mut rx = data.family_hub.subscribe(fid);
        let event_tx_clone = event_tx.clone();

        tokio::spawn(async move {
            while let Ok(event) = rx.recv().await {
                if event_tx_clone.send(event).await.is_err() {
                    break;
                }
            }
        });
    }

    // Ping interval to maintain connectivity through carrier timeouts
    let mut ping_interval = interval(Duration::from_secs(30));

    // Spawn async background pump task on Actix runtime
    actix_web::rt::spawn(async move {
        loop {
            tokio::select! {
                // Outgoing: Send realtime events received on internal channel to client
                Some(event) = event_rx.recv() => {
                    match serde_json::to_string(&event) {
                        Ok(json) => {
                            if session.text(json).await.is_err() {
                                debug!("Failed to send text frame to WS client; closing connection.");
                                break;
                            }
                        }
                        Err(e) => {
                            error!("Failed to serialize RealtimeEvent for WS: {}", e);
                        }
                    }
                }

                // Heartbeat: Send periodic Ping to keep carrier connection alive
                _ = ping_interval.tick() => {
                    if session.ping(b"heartbeat").await.is_err() {
                        debug!("Failed to send ping to WS client; connection lost.");
                        break;
                    }
                }

                // Incoming: Receive client frames (Pong / close / text)
                Some(msg) = msg_stream.next() => {
                    match msg {
                        Ok(actix_ws::Message::Ping(bytes)) => {
                            if session.pong(&bytes).await.is_err() {
                                break;
                            }
                        }
                        Ok(actix_ws::Message::Pong(_)) => {
                            debug!("Received pong from client user_id={}", user_id);
                        }
                        Ok(actix_ws::Message::Close(reason)) => {
                            info!("WebSocket client closed connection: {:?}", reason);
                            let _ = session.clone().close(reason).await;
                            break;
                        }
                        Ok(actix_ws::Message::Text(text)) => {
                            debug!("Received text frame from WS client: {}", text);
                        }
                        Ok(actix_ws::Message::Binary(_)) => {}
                        Ok(actix_ws::Message::Nop) => {}
                        Ok(_) => {}
                        Err(e) => {
                            warn!("WebSocket error for user_id={}: {:?}", user_id, e);
                            break;
                        }
                    }
                }

                else => break,
            }
        }

        let _ = session.close(None).await;
        info!("WebSocket session finished for user_id={}", user_id);
    });

    Ok(response)
}
