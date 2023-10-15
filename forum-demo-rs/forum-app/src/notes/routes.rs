use axum::extract::State;
use axum::http::StatusCode;
use axum::{
    routing::{get, post},
    Json, Router,
};
use sqlx::Pool;
use std::sync::Arc;

use super::{Note, NoteBase, NoteSer};

async fn get_notes(State(serv): State<Arc<NoteSer>>) -> Result<Json<Vec<Note>>, StatusCode> {
    match serv.get_all_notes().await {
        Some(notes) => Ok(Json(notes)),
        None => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn post_note(
    State(serv): State<Arc<NoteSer>>,
    Json(base): Json<NoteBase>,
) -> Result<(StatusCode, Json<Note>), StatusCode> {
    match serv.create_note(base).await {
        Some(note) => Ok((StatusCode::CREATED, Json(note))),
        None => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// fix dep overreaching
pub fn router(pool: Pool<sqlx::Postgres>) -> Router {
    let state = Arc::new(NoteSer::new(pool));

    Router::new()
        .route("/", get(get_notes))
        .route("/", post(post_note))
        .with_state(state)
}
