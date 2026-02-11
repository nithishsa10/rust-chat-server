use redis::{
    aio::ConnectionManager,
    AsyncCommands,
};
use uuid::Uuid;
use chrono::Utc;
use serde_json;

use crate::models::session::Session;
use crate::error::{Result, AppError};

const SESSION_TTL_SECS: usize = 604_800;
const TRYING_TTL_SECS: usize = 5;

pub async fn create_connection_manager(url: &str) -> Result<ConnectionManager> {
    let client = redis::Client::open(url)
        .map_err(|e| AppError::Internal(format!("Redis cliene: {e}").into()))?;
    ConnectionManager::new(client)
        .await
        .map_err(|e| AppError::Internal(format!("Redis pool: {e}").into()))
}