mod models;
mod repositories;
mod error;
mod handlers;
mod db;
mod websocket;
mod service;
mod utils;
mod config;

use axum::{
    routing::{get, post, put, delete},
    Router
};
use tracing_subscriber::{fmt, EnvFilter};
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;
use std::str::FromStr;

use db::DbPool;
use websocket::hub::Hub;
use config::Config;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub hub: Hub
}

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();

    let env_filter = EnvFilter::from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(env_filter)
        .init();

    let cfg = Config::from_env().expect("Failed to load config");
    tracing::info!("Starting rust-chat-server on {}:{}", cfg.host, cfg.port);

    let pg = db::posgresql::create_pool(&cfg.database_url)
        .await
        .expect("Failed to connect to postgres database");
    
    let redis = db::redis::create_client(&cfg.redis_url)
        .expect("Failed to connect to redis");
    
    let state = AppState {
        pool: DbPool { pool: pg, redis },
        hub: Hub::new()
    };

    let app = Router::new()
        .route("/api/auth/register", post(handlers::auth::register))
        .route("/test", get(test()))

        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                    axum::http::Method::PUT,
                    axum::http::Method::DELETE,
                    axum::http::Method::OPTIONS,
                ])
                .allow_headers([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                ]),
        )
        .with_state(state);

    let addr = format!("{}:{}", cfg.host, cfg.port)
        .parse::<SocketAddr>()
        .expect("Invalid host:port");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on http://{addr}");
    tracing::info!("WebSocket: ws://{addr}/ws?token=<jwt>");
    tracing::info!("Health:    http://{addr}/health");

    axum::serve(listener, app).await.unwrap();
}

fn test() -> String {
    String::from("Hello World")
}