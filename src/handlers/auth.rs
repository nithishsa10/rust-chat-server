use axum::{extract::State, Json};
use serde_json::json;
use serde::Deserialize;

use crate::AppState;
use crate::models::user::{self, RegisterRequest};
use crate::error::Result;
use crate::services::auth_service;
use crate::middleware::auth::AuthUser;


pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<serde_json::Value>> {
    let (user, token) = auth_service::register(&state.pool, &req).await?;
    Ok(Json(json!({"user": user, "token": token})))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<serde_json::Value>> {
    let (user, token) = auth_service::login(&state.pool, &req).await?;
    Ok(Json(json!({"user": user, "token": token})))
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest { pub refresh_token: String }

pub async fn refresh(
    Json(req): Json<RefreshRequest>
) -> Result<Json<serde_json::Value>> {
    let token = auth_service::refresh_token(&req.refresh_token).await?;
    Ok(Json(json!({"token": token})))
}

pub async fn logout(
    State(_state): State<AppState>,
    _auth: AuthUser,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(json!({ "message": "Logged out" })))
}

pub async fn me(State(state): State<AppState>, auth: AuthUser) -> Result<Json<serde_json::Value>> {
    let user_id = auth.claims().user_id()?;
    let user = auth_service::get_current_user(&state.pool, user_id).await?;
    Ok(Json(json!({ "user": user })))
}
