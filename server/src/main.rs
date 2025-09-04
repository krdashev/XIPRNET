//! XIPRNET Server
//! 
//! Main server binary for the XIPRNET messaging system

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::info;

mod api;
mod auth;
mod storage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("Starting XIPRNET server...");
    
    // Create CORS layer
    let cors = CorsLayer::permissive();
    
    // Create router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/auth/register", post(api::auth::register))
        .route("/api/v1/auth/login", post(api::auth::login))
        .route("/api/v1/messages", post(api::messages::send_message))
        .route("/api/v1/messages", get(api::messages::get_messages))
        .route("/api/v1/sync", post(api::sync::sync_messages))
        .layer(cors);
    
    // Bind to address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("TCP listener bound successfully");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
