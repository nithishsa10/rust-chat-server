use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// fromRow used for sqlx 
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id:             Uuid,
    pub username:       String,
    pub email:          String,
    pub password_hash:  String,
    pub display_name:   Option<String>,
    pub avatar_url:     Option<String>,
    pub created_at:     DateTime<Utc>,
    pub updated_at:      DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    pub id:             Uuid,
    pub username:       String,
    pub display_name:   Option<String>,
    pub avatar_url:     Option<String>,
    pub created_at:     DateTime<Utc>,
    pub updated_at:      DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
       Self { 
            id:             u.id,
            username:       u.username,
            display_name:   u.display_name,
            avatar_url:     u.avatar_url,
            created_at:     u.created_at,
            updated_at:     u.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username:       String,
    pub email:          String,
    pub password:       String,
    pub display_name:   Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username:       String,
    pub password:       String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub display_name:   Option<String>,
    pub avatar_url:     Option<String>,
}
