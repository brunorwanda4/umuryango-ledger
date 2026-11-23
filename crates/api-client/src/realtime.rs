use futures_util::{SinkExt, StreamExt};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{error, info, warn};
pub use umuryango_core_domain::RealtimeEvent;

pub struct RealtimeHandle {
    is_running: Arc<AtomicBool>,
}

impl RealtimeHandle {
    pub fn disconnect(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }
}

pub struct RealtimeClient;

impl RealtimeClient {
    /// Connects to the backend WebSocket endpoint with automatic exponential backoff reconnect.
    pub fn connect(
        ws_url: String,
        token: String,
    ) -> (RealtimeHandle, mpsc::UnboundedReceiver<RealtimeEvent>) {
        let (tx, rx) = mpsc::unbounded_channel();
        let is_running = Arc::new(AtomicBool::new(true));
        let running_clone = is_running.clone();

        tokio::spawn(async move {
            let mut backoff_secs = 1;
            let target_url = if ws_url.contains("/api/v1/ws") {
                format!("{}?token={}", ws_url, token)
            } else {
                format!("{}/api/v1/ws?token={}", ws_url.trim_end_matches('/'), token)
            };

            while running_clone.load(Ordering::SeqCst) {
                info!("RealtimeClient attempting connection to {}", target_url);

                match connect_async(&target_url).await {
                    Ok((ws_stream, _)) => {
                        info!("RealtimeClient connected successfully");
                        backoff_secs = 1; // Reset backoff on successful connect

                        let (mut write, mut read) = ws_stream.split();

                        while running_clone.load(Ordering::SeqCst) {
                            tokio::select! {
                                msg = read.next() => {
                                    match msg {
                                        Some(Ok(Message::Text(text))) => {
                                            match serde_json::from_str::<RealtimeEvent>(&text) {
                                                Ok(event) => {
                                                    if tx.send(event).is_err() {
                                                        // Receiver closed
                                                        return;
                                                    }
                                                }
                                                Err(e) => {
                                                    warn!("Failed to deserialize RealtimeEvent: {:?}; raw: {}", e, text);
                                                }
                                            }
                                        }
                                        Some(Ok(Message::Ping(payload))) => {
                                            let _ = write.send(Message::Pong(payload)).await;
                                        }
                                        Some(Ok(Message::Pong(_))) => {}
                                        Some(Ok(Message::Close(_))) => {
                                            warn!("WebSocket server closed connection");
                                            break;
                                        }
                                        Some(Err(e)) => {
                                            warn!("WebSocket read error: {:?}", e);
                                            break;
                                        }
                                        None => {
                                            warn!("WebSocket stream EOF");
                                            break;
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!(
                            "WebSocket connection failed: {:?}. Retrying in {}s...",
                            e, backoff_secs
                        );
                    }
                }

                if !running_clone.load(Ordering::SeqCst) {
                    break;
                }

                sleep(Duration::from_secs(backoff_secs)).await;
                backoff_secs = (backoff_secs * 2).min(30);
            }
        });

        (RealtimeHandle { is_running }, rx)
    }
}
