use axum::{
    extract::State, http::StatusCode, response::Json as ResponseJson, routing::post, Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod config;
mod models;
fn main() {
    println!("Hello, world!");
}
