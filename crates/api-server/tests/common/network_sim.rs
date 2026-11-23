use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkMode {
    Online = 0,
    Offline = 1,
    DropAfterForwardOnce = 2,
}

impl From<u8> for NetworkMode {
    fn from(val: u8) -> Self {
        match val {
            0 => NetworkMode::Online,
            1 => NetworkMode::Offline,
            2 => NetworkMode::DropAfterForwardOnce,
            _ => NetworkMode::Online,
        }
    }
}

pub struct FaultyNetworkProxy {
    addr: SocketAddr,
    mode: Arc<AtomicU8>,
    _handle: JoinHandle<()>,
}

impl FaultyNetworkProxy {
    pub async fn new(target_addr: SocketAddr) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("Failed to bind proxy listener");
        let addr = listener
            .local_addr()
            .expect("Failed to get proxy local addr");
        let mode = Arc::new(AtomicU8::new(NetworkMode::Online as u8));
        let mode_clone = mode.clone();

        let handle = tokio::spawn(async move {
            loop {
                let Ok((client_stream, _)) = listener.accept().await else {
                    break;
                };

                let current_mode: NetworkMode = mode_clone.load(Ordering::SeqCst).into();
                let mode_atomic = mode_clone.clone();

                tokio::spawn(async move {
                    match current_mode {
                        NetworkMode::Offline => {
                            // Immediately terminate the client connection without forwarding
                            drop(client_stream);
                        }
                        NetworkMode::DropAfterForwardOnce => {
                            // Connect to target server, forward the request, then drop client before returning response
                            if let Ok(server_stream) = TcpStream::connect(target_addr).await {
                                let (mut c_read, _) = client_stream.into_split();
                                let (_, mut s_write) = server_stream.into_split();

                                let mut buf = vec![0u8; 16384];
                                if let Ok(n) = c_read.read(&mut buf).await {
                                    if n > 0 {
                                        let _ = s_write.write_all(&buf[..n]).await;
                                        // Forward any remaining body bytes if Content-Length exists
                                        let header_str = String::from_utf8_lossy(&buf[..n]);
                                        let mut expected_len = None;
                                        for line in header_str.lines() {
                                            if line
                                                .to_ascii_lowercase()
                                                .starts_with("content-length:")
                                            {
                                                if let Some(val) = line.split(':').nth(1) {
                                                    if let Ok(len) = val.trim().parse::<usize>() {
                                                        expected_len = Some(len);
                                                    }
                                                }
                                            }
                                        }

                                        if let Some(content_len) = expected_len {
                                            if let Some(body_start) = header_str.find("\r\n\r\n") {
                                                let body_read = n - (body_start + 4);
                                                let mut remaining =
                                                    content_len.saturating_sub(body_read);
                                                while remaining > 0 {
                                                    let mut b = vec![0u8; remaining.min(8192)];
                                                    if let Ok(rn) = c_read.read(&mut b).await {
                                                        if rn == 0 {
                                                            break;
                                                        }
                                                        let _ = s_write.write_all(&b[..rn]).await;
                                                        remaining = remaining.saturating_sub(rn);
                                                    } else {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                        // Wait a brief moment for the target server to process
                                        tokio::time::sleep(tokio::time::Duration::from_millis(150))
                                            .await;
                                    }
                                }
                            }
                            // Revert back to Online for subsequent calls so recovery works cleanly
                            mode_atomic.store(NetworkMode::Online as u8, Ordering::SeqCst);
                        }
                        NetworkMode::Online => {
                            if let Ok(server_stream) = TcpStream::connect(target_addr).await {
                                let (mut c_read, mut c_write) = client_stream.into_split();
                                let (mut s_read, mut s_write) = server_stream.into_split();

                                let client_to_server = tokio::io::copy(&mut c_read, &mut s_write);
                                let server_to_client = tokio::io::copy(&mut s_read, &mut c_write);

                                let _ = tokio::try_join!(client_to_server, server_to_client);
                            }
                        }
                    }
                });
            }
        });

        Self {
            addr,
            mode,
            _handle: handle,
        }
    }

    pub fn set_mode(&self, mode: NetworkMode) {
        self.mode.store(mode as u8, Ordering::SeqCst);
    }

    #[allow(dead_code)]
    pub fn mode(&self) -> NetworkMode {
        self.mode.load(Ordering::SeqCst).into()
    }

    #[allow(dead_code)]
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn url(&self) -> String {
        format!("http://{}", self.addr)
    }
}
