use sqlx::PgPool;
use tower_http::trace;
use uuid::Uuid;
use crate::models::user::User;
use crate::error::{AppError, Result};
pub async fn create_user(
    pool: &PgPool,
    username: &str,
    email: &str,
    password_hash: &str,
    display_name: Option<&str>
) -> Result<User> {
    let user = sqlx::query_as::<_, User> (
        r#"
         INSERT INTO users (username, email, password_hash, display_name)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(username)
    .bind(email)
    .bind(password_hash)
    .bind(display_name)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(format!("Failed to create user: {}", e)))?;
    Ok(user)
}

pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>> {
    Ok(sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?)
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
    Ok(sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await?
    )
}

pub async fn get_user_by_username(pool: &PgPool, username: &str) -> Result<Option<User>> {
    let user_data = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await?;
    return Ok(user_data)
}

pub async fn update_user_profile(
    pool: &PgPool,
    user_id: &str,
    display_name: Option<&str>,
    avatar_url: Option<&str>
) -> Result<User> {
    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET display_name = COALESCE($2, display_name),
            avatar_url   = COALESCE($3, avatar_url),
            updated_at   = NOW()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(display_name)
    .bind(avatar_url)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(format!("Failed to update user: {}", e)))?;
    Ok(user)
}

pub async fn search_user(pool: &PgPool, query: &str, limit: i32) -> Result<Vec<User>> {
    let pattern = format!("%{}%", query.to_lowercase());
    Ok(sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE LOWER(username) LIKE $1 OR LOWER(display_name) LIKE $1 LIMIT $2",
    )
    .bind(pattern)
    .bind(limit)
    .fetch_all(pool)
    .await?)
}