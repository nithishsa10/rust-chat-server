use uuid::Uuid;
use serde_json::Value;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
// use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, sqlx::FromRow,)]
pub struct Message {
    pub id:             Uuid,
    pub rood_id:        Uuid,
    pub sender_id:      Uuid,
    pub content:        String,
    pub message_type:   String,
    pub meta_data:      Option<Value>,
    // pub is_read:        bool,
    pub created_at:     DateTime<Utc>,
    pub update_at:      DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow,)]
pub struct DirectMessage {
    pub id:             Uuid,
    pub sender_id:      Uuid,
    pub recipient_id:   Uuid,
    pub content:        String,
    pub is_read:        bool,
    pub created_at:     DateTime<Utc>,
    pub update_at:      DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MessageEvent {
    pub message_id:     Uuid,
    pub room_id:        Uuid,
    pub user:           MessageUser,
    pub content:        String,
    pub timestamp:      DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MessageUser {
    pub id:             Uuid,
    pub username:       String,
    pub display_name:   Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}