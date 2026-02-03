pub mod posgresql;
// pub mod redis;

use sqlx::PgPool;
use redis::aio::ConnectionManager;

#[derive(Clone)]
pub struct DbPool {
    pub pool: PgPool,
    pub redis: ConnectionManager
}
