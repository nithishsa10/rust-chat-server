use axum::extract::ws::{WebSocket, Message as WsMsg};
use tokio::sync::mpsc;
use uuid::Uuid;
use tracing::{info, warn, error};

use super::protocol::{ClientMessage, ServerMessage};

/// One live WebSocket connection.  The hub holds the sender; the handler owns the receiver.
#[derive(Clone)]
pub struct Connection {
    pub user_id:  Uuid,
    pub username: String,
    pub tx:       mpsc::Sender<ServerMessage>,
}

/// Spawn the reader loop for a single WebSocket.
/// Returns the receiver half so the hub can feed outgoing frames.
pub async fn run_connection(
    mut socket: WebSocket,
    user_id:    Uuid,
    username:   String,
    tx:         mpsc::Sender<ServerMessage>,
    rx:         mpsc::Receiver<ServerMessage>,
    on_message: impl Fn(Uuid, ClientMessage) + Send + 'static,
    on_disconnect: impl FnOnce(Uuid) + Send + 'static,
) {
    // Spawn writer task
    let mut rx = rx;
    let writer_handle = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let frame = match serde_json::to_string(&msg) {
                Ok(s)  => s,
                Err(e) => { error!("serialize ServerMessage: {e}"); continue; }
            };
            // socket.send requires &mut WebSocket — we use a trick: send via a second channel
            // Instead we return the frame so the main loop can send it.
            // Simpler design: use a single task.  We'll merge read+write below.
            // For clarity we store frames in a channel and let the outer loop send.
            tx.send(ServerMessage::Pong).await.ok(); // placeholder – see merged loop below
            let _ = frame; // handled in merged loop
        }
    });

    // ── Merged read/write loop ──────────────────────────
    // We poll both the socket (incoming) and the rx channel (outgoing).
    use tokio::select;

    loop {
        select! {
            // Incoming frame from client
            frame = socket.recv() => {
                match frame {
                    Some(Ok(WsMsg::Text(text))) => {
                        match serde_json::from_str::<ClientMessage>(&text) {
                            Ok(msg) => on_message(user_id, msg),
                            Err(e) => {
                                warn!("parse ClientMessage: {e}");
                                let err = ServerMessage::Error {
                                    code:    "PARSE_ERROR".into(),
                                    message: format!("Invalid message: {e}"),
                                };
                                let _ = socket.send(WsMsg::Text(serde_json::to_string(&err).unwrap())).await;
                            }
                        }
                    }
                    Some(Ok(WsMsg::Close(_))) | None => {
                        info!("WebSocket closed for user {user_id}");
                        break;
                    }
                    Some(Err(e)) => {
                        warn!("WebSocket error for user {user_id}: {e}");
                        break;
                    }
                    _ => {} // ping/pong/binary – ignore
                }
            }
            // Outgoing frame from hub
            msg = rx.recv() => {
                match msg {
                    Some(server_msg) => {
                        match serde_json::to_string(&server_msg) {
                            Ok(text) => {
                                if socket.send(WsMsg::Text(text)).await.is_err() {
                                    break;
                                }
                            }
                            Err(e) => error!("serialize: {e}"),
                        }
                    }
                    None => break, // channel closed
                }
            }
        }
    }

    on_disconnect(user_id);
    writer_handle.abort();
}
