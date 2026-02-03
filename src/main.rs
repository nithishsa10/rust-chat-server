mod models;
mod repositories;
mod error;
mod handlers;
mod db;
mod websocket;
mod service;
mod utils;

use db::DbPool;
use websocket::hub::Hub;
pub struct AppState {
    pub pool: DbPool,
    pub hub: Hub
}
// #[tokio::main]
fn main() {

}