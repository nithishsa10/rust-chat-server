use axum::{extract::State, Json};

use crate::AppState;
use crate::models::auth::RegisterPayload;
use crate::error::Result;

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterPayload>
) -> Result<Json<serde_json::Value>> {
    let (user, token) = auth_service::register(&state.db, &req).await?;
    Ok(Json(json!({"user": user, "token": token})))
}