use axum::{
    extract::State, http::StatusCode, response::Json as ResponseJson, routing::post, Router,
};

mod config;
mod error;
mod handlers;
mod models;
mod services;

use config::Config;
use error::AppError;
use models::*;
use services::SolanaService;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env();
    let service = Arc::new(SolanaService::new(config)?);

    let app = Router::new()
        .route("/keypair", post(handlers::generate_keypair))
        .route("/token/create", post(handlers::create_token))
        .route("/token/mint", post(handlers::mint_token))
        .route("/message/sign", post(handlers::sign_message))
        .route("/message/verify", post(handlers::verify_message))
        .route("/send/sol", post(handlers::send_sol))
        .route("/send/token", post(handlers::send_token))
        .layer(CorsLayer::permissive())
        .with_state(service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("🚀 Solana HTTP Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;
    Ok(())
}
