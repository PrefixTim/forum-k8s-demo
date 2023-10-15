use axum::routing::get;
use axum::{Json, Router};
use chrono::{DateTime, Local};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;

mod notes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr =
        format!("{}:{}", get_env("ADDR", "0.0.0.0"), get_env("PORT", "8080")).parse()?;
    let db_url: String = env::var("DB_URL").expect("Need DB_URL env variable");

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await?;

    let app = Router::new()
        .route("/time", get(time_json))
        .nest("/notes", notes::router(pool));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

fn get_env(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or(default.to_owned())
}

async fn time_json() -> Json<DateTime<Local>> {
    Json(chrono::offset::Local::now())
}
