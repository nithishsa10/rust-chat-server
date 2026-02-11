use uuid::Uuid;
use chrono::Utc;

use crate::db::DbPool;
use crate::models::user::RegisterRequest;
use crate::models::session::{Session, AuthTokens};
use crate::models::user::{self, UserResponse};
use crate::error::{Result, AppError};
use crate::repositories::user_repo;
use crate::utils::{jwt, password};

// async fn store_session(pool: &DbPool, user_id: Uuid, username: &str) -> Result<()> {
//     let session = Session {
//         user_id,
//         username:    username.to_string(),
//         created_at:  Utc::now(),
//         last_active: Utc::now(),
//     };
//     let mut redis = pool.redis.clone();
    
//     // rd::set_session(&mut redis, &generate_session_id(), &session).await
// }

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
        expires_at:    access_exp,
        refresh_expires: refresh_exp
    })
}
pub async fn register(db: &DbPool, req: &RegisterRequest) -> Result<(UserResponse, AuthTokens)> {
    if req.username.len() < 3 || req.username.len() > 50 {
        return Err(AppError::BadRequest("Username must be 3-50 characters".into()));
    }
    if req.password.len() < 8 {
        return Err(AppError::BadRequest("Password must be at least 8 characters".into()));
    }
    if !is_valid_email(&req.email).await {
        return Err(AppError::BadRequest("Invalid email address".into()));
    }
    if user_repo::get_user_by_username(&db.pg, &req.username).await?.is_some() {
        return Err(AppError::Conflict("Username already taken".into()));
    }
    if user_repo::get_user_by_email(&db.pg, &req.email).await?.is_some() {
        return Err(AppError::Conflict("Email already registered".into()));
    }

    let hashed = password::hash_password(&req.password)?;
    let user = user_repo::create_user(&db.pg, &req.username, &req.email, &hashed, req.display_name.as_deref()).await?;
    let token = make_tokens(user.id, &user.username)?;
    // store_session(pool, user_id, username);
    Ok((user.into(), token))
}

pub async fn login(db: &DbPool, req: &RegisterRequest) -> Result<(UserResponse, AuthTokens)> {
    let user = user_repo::get_user_by_email(&db.pg, &req.email).await?
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".into()))?;

    if !password::verify_password(&req.password, &user.password_hash)? {
        return Err(AppError::Unauthorized("Invalid password".into()));
    }
    let token = make_tokens(user.id, &user.username)?;
    Ok((user.into(), token))
}


async fn is_valid_email(email: &str) -> bool {
    if email.contains("@") {
        return true
    }
    false
}

pub async fn refresh_token(token: &str) -> Result<AuthTokens> {
    let (secret, _, _) = read_jwt_env();
    let claims = jwt::verify_token(token, &secret)?;
    if claims.token_type != "refresh" {
        return Err(AppError::Unauthorized("Not a valid refresh token".into()));
    }
    return make_tokens(claims.user_id()?, &claims.username);
}
