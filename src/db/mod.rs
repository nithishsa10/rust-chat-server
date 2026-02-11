pub mod posgresql;
pub mod redisdb;

use sqlx::PgPool;
use redis::aio::ConnectionManager;

#[derive(Clone)]
pub struct DbPool {
    pub pg: PgPool,
    pub redis: ConnectionManager
}
