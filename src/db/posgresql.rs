use sqlx::{PgPool, Error};
use crate::error::{AppError, Result};

pub async fn create_pool(url: &str) -> Result<PgPool> {
    PgPool::connect(url)
    .await
    .map_err(|e| AppError::Database(e))
}
