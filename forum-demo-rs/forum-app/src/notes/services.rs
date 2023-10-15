use super::{Note, NoteBase};
use sqlx::PgPool;

pub struct NoteSer {
    pool: PgPool,
    // nid: NoteId,
}

impl NoteSer {
    pub fn new(pool: PgPool) -> Self {
        NoteSer { pool }
    }
    pub async fn create_note(&self, base: NoteBase) -> Option<Note> {
        let note = Note::new(base.content);
        let res = sqlx::query(
            r#"
            INSERT INTO notes (id, content)
            VALUES ($1, $2)
            "#,
        )
        .bind(&note.id)
        .bind(&note.content)
        .execute(&self.pool)
        .await;

        match res {
            Ok(_) => Some(note),
            Err(_err) => {
                //log err
                None
            }
        }
    }

    pub async fn get_all_notes(&self) -> Option<Vec<Note>> {
        let res = sqlx::query_as::<_, Note>("SELECT * FROM notes")
            .fetch_all(&self.pool)
            .await;
        match res {
            Ok(quotes) => Some(quotes),
            Err(_err) => {
                //log
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn create_get() {
    //     let mut ser = NoteSer::new();
    //     let content = "test".to_string();
    //     assert_eq!(0, ser.create_note(NoteBase{content: content.clone()}));
    //     assert_eq!(Some(&Note{id: 0, content }), ser.get_note(0));
    // }
}
