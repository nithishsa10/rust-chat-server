use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Room {
    pub id:             Uuid,
    pub name:           String,
    pub description:    String,
    pub is_private:     bool,
    pub created_at:     DateTime<Utc>,
    pub update_at:      DateTime<Utc>,
    pub created_by:     Uuid
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct RoomMember {
    pub rood_id:    Uuid,
    pub user_id:    Uuid,
    pub role:       String,
    pub join_at:    DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRoomRequest {
    pub name:           String,
    pub description:    String,
    pub is_private:     bool
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoomRequest {
    pub name:           Option<String>,
    pub description:    Option<String>,
    pub is_private:     Option<bool>,
}