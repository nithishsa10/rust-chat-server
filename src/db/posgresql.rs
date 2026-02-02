use sqlx::{PgPool, Error};

pub async fn create_pool(url: &str) -> Result<PgPool, Error> {
    PgPool::connect(url)
    .await
    .map_err(|e| AppError::DatabaseError(format!("Postgres connection error: {}", e)))
}
