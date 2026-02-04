/// In-memory connection hub.  Keeps track of every live WebSocket and which rooms
/// each user has joined.  Provides broadcast helpers used by the WebSocket handler.
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use uuid::Uuid;
use tracing::{info, warn};

use super::protocol::ServerMessage;
use super::connection::Connection;

#[derive(Clone)]
pub struct Hub {
    inner: Arc<Mutex<HubInner>>,
}

struct HubInner {
    /// user_id → active connection (sender channel)
    connections: HashMap<Uuid, Connection>,
    /// room_id → set of user_ids currently in that room
    rooms: HashMap<Uuid, HashSet<Uuid>>,
}

impl Hub {
    pub fn new() -> Self {
        Hub {
            inner: Arc::new(Mutex::new(HubInner {
                connections: HashMap::new(),
                rooms:       HashMap::new(),
            })),
        }
    }

    /// Register a new connection and return its channel pair.
    pub fn register(&self, user_id: Uuid, username: String) -> (mpsc::Sender<ServerMessage>, mpsc::Receiver<ServerMessage>) {
        let (tx, rx) = mpsc::channel(64);
        let conn = Connection { user_id, username, tx: tx.clone() };
        self.inner.lock().unwrap().connections.insert(user_id, conn);
        info!("Hub: registered {user_id}");
        (tx, rx)
    }

    /// Remove a connection (on disconnect).
    pub fn disconnect(&self, user_id: Uuid) {
        let mut inner = self.inner.lock().unwrap();
        inner.connections.remove(&user_id);
        for members in inner.rooms.values_mut() {
            members.remove(&user_id);
        }
        info!("Hub: disconnected {user_id}");
    }

    /// Add user to a room and notify the room.
    pub fn join_room(&self, room_id: Uuid, user_id: Uuid, username: &str, display_name: Option<&str>) {
        let mut inner = self.inner.lock().unwrap();
        inner.rooms.entry(room_id).or_default().insert(user_id);

        let joined_msg = ServerMessage::UserJoined {
            room_id,
            user: super::protocol::WsUser {
                id:           user_id,
                username:     username.to_string(),
                display_name: display_name.map(|s| s.to_string()),
            },
        };
        // Notify everyone else in the room
        Self::broadcast_inner(&inner, room_id, &joined_msg, Some(user_id));

        // Send online-users list to the joiner
        let online: Vec<String> = inner.rooms.get(&room_id)
            .map(|s| s.iter().map(|id| id.to_string()).collect())
            .unwrap_or_default();
        if let Some(conn) = inner.connections.get(&user_id) {
            let _ = conn.tx.try_send(ServerMessage::OnlineUsers { room_id, user_ids: online });
        }
    }

    /// Remove user from a room and notify.
    pub fn leave_room(&self, room_id: Uuid, user_id: Uuid) {
        let mut inner = self.inner.lock().unwrap();
        if let Some(members) = inner.rooms.get_mut(&room_id) {
            members.remove(&user_id);
        }
        let left_msg = ServerMessage::UserLeft { room_id, user_id };
        Self::broadcast_inner(&inner, room_id, &left_msg, None);
    }

    /// Broadcast a message to every user in a room (optionally skipping one).
    pub fn broadcast_to_room(&self, room_id: Uuid, msg: &ServerMessage, skip_user: Option<Uuid>) {
        let inner = self.inner.lock().unwrap();
        Self::broadcast_inner(&inner, room_id, msg, skip_user);
    }

    fn broadcast_inner(inner: &HubInner, room_id: Uuid, msg: &ServerMessage, skip: Option<Uuid>) {
        if let Some(members) = inner.rooms.get(&room_id) {
            for &uid in members {
                if skip == Some(uid) { continue; }
                if let Some(conn) = inner.connections.get(&uid) {
                    if conn.tx.try_send(msg.clone()).is_err() {
                        warn!("Hub: channel full or closed for {uid}");
                    }
                }
            }
        }
    }

    /// Send a message to a single user (for DMs / pong).
    pub fn send_to_user(&self, user_id: Uuid, msg: &ServerMessage) {
        let inner = self.inner.lock().unwrap();
        if let Some(conn) = inner.connections.get(&user_id) {
            let _ = conn.tx.try_send(msg.clone());
        }
    }
}
