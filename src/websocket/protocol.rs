/// Typed WebSocket frames ─ both directions.
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Client → Server 
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    JoinRoom   { room_id: Uuid },
    LeaveRoom  { room_id: Uuid },
    Message    { room_id: Uuid, content: String },
    Typing     { room_id: Uuid, is_typing: bool },
    Dm         { recipient_id: Uuid, content: String },
    Ping,
}

//  Server → Client 
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    Message {
        message_id: Uuid,
        room_id:    Uuid,
        user:       WsUser,
        content:    String,
        timestamp:  DateTime<Utc>,
    },
    UserJoined {
        room_id: Uuid,
        user:    WsUser,
    },
    UserLeft {
        room_id: Uuid,
        user_id: Uuid,
    },
    Typing {
        room_id:    Uuid,
        user_id:    Uuid,
        username:   String,
        is_typing:  bool,
    },
    Dm {
        from:      WsUser,
        content:   String,
        timestamp: DateTime<Utc>,
    },
    OnlineUsers {
        room_id: Uuid,
        user_ids: Vec<String>,
    },
    Error {
        code:    String,
        message: String,
    },
    Pong,
}

#[derive(Debug, Serialize, Clone)]
pub struct WsUser {
    pub id:           Uuid,
    pub username:     String,
    pub display_name: Option<String>,
}
