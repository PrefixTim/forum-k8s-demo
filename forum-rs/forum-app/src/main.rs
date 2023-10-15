use std::net::SocketAddr;

use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::get;
use chrono::{DateTime, Local};

fn get_env(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or(default.to_owned())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = format!("{}:{}", get_env("ADDR", "0.0.0.0"), get_env("PORT", "8080")).parse()?;

    let app = Router::new().route("/", get(health)).route("/time", get(time_json));

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
    Ok(())
}

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn time_json() -> Json<DateTime<Local>> {
    Json(chrono::offset::Local::now())
}