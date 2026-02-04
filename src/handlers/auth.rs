use axum::{extract::State, Json};
use serde_json::json;

use crate::AppState;
use crate::models::user::RegisterRequest;
use crate::error::Result;
use crate::services::auth_service;


pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>
) -> Result<Json<serde_json::Value>> {
    let (user, token) = auth_service::register(&state.pool, &req).await?;
    Ok(Json(json!({"user": user, "token": token})))
}