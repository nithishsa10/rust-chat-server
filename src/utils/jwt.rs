use uuid::Uuid;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};    
use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};

use crate::error::{AppError, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub:          String,   // user_id as string
    pub username:     String,
    pub exp:          i64,      // expiry timestamp
    pub iat:          i64,      // issued-at timestamp
    pub token_type:   String,   // "access" | "refresh"
}


impl Claims {
    pub fn user_id(&self) -> Result<Uuid> {
        Uuid::parse_str(&self.sub)
            .map_err(|_| AppError::Unauthorized("Invalid user_id in token".into()))
    }
}

pub fn create_token(
    user_id: Uuid,
    username: &str,
    secret: &str,
    expiry_secs: i64,
    token_type: &str,
) -> Result<String> {
     let now = Utc::now().timestamp();
    let claims = Claims {
        sub:        user_id.to_string(),
        username:   username.to_string(),
        exp:        now + expiry_secs,
        iat:        now,
        token_type: token_type.into(),
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(AppError::from)
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims> {
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| AppError::Unauthorized("Invalid or expired token".into()))?
    .claims;

    Ok(claims)
} 