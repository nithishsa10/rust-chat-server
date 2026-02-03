use uuid::Uuid;

use crate::models::auth::RegisterPayload;
use crate::models::user::{self, UserResponse};
use crate::error::Result;
use crate::repositories::user_repo;
use crate::utils::{jwt, password};

async fn store_session(pool: &DbPool, user_id: Uuid, username: &str) -> Result<()> {
    let session = Session {
        user_id,
        username:    username.to_string(),
        created_at:  Utc::now(),
        last_active: Utc::now(),
    };
    let mut redis = pool.redis.clone();
    // rd::set_session(&mut redis, &generate_session_id(), &session).await
}

fn read_jwt_env() -> (String, i64, i64) {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
    let access_exp  = std::env::var("JWT_EXPIRY_SECS").unwrap_or_else(|_| "86400".into()).parse().unwrap_or(86400);
    let refresh_exp = std::env::var("JWT_REFRESH_EXPIRY_SECS").unwrap_or_else(|_| "604800".into()).parse().unwrap_or(604800);
    (secret, access_exp, refresh_exp)

}
fn make_tokens(user_id: Uuid, username: &str) -> Result<AuthTokens> {
    let (secret, access_exp, refresh_exp) = read_jwt_env();
    Ok(AuthTokens {
        access_token:  jwt::create_token(user_id, username, &secret, access_exp, "access")?,
        refresh_token: jwt::create_token(user_id, username, &secret, refresh_exp, "refresh")?,
        expires_in:    access_exp,
    })
}
pub async fn register(db: &PgPool, req: &RegisterPayload) -> Result<(UserResponse, AuthTokens)> {
    if req.username.len() < 3 || req.username.len() > 50 {
        return Err(AppError::BadRequest("Username must be 3-50 characters".into()));
    }
    if req.password.len() < 8 {
        return Err(AppError::BadRequest("Password must be at least 8 characters".into()));
    }
    if !is_valid_email(&req.email) {
        return Err(AppError::BadRequest("Invalid email address".into()));
    }
    if user_repo::get_user_by_username(&pool.pg, &req.username).await?.is_some() {
        return Err(AppError::Conflict("Username already taken".into()));
    }
    if user_repo::get_user_by_email(&pool.pg, &req.email).await?.is_some() {
        return Err(AppError::Conflict("Email already registered".into()));
    }

    let hashed = password::hash_password(&req.password).await?;
    let user = user_repo::create_user(&pool.pg, &req.username, &req.email, &hashed, &req.display_name).await?;
    let token = make_tokens(user.user, &user.username).await?;
    // store_session(pool, user_id, username);
    Ok((user.into(), token))
}

async fn is_valid_email(email: &str) -> bool {
    regex::Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap().is_match(email)
}

