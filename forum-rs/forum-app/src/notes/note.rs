use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, FromRow)]
pub struct Note {
    pub id: uuid::Uuid,
    pub content: String,
    // inserted_at: chrono::DateTime<chrono::Utc>
    // lang: String
}

impl Note {
    pub fn new(content: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            content,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NoteBase {
    pub content: String,
    // lang: String
}
