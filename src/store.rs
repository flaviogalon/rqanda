use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

use crate::error;
use crate::types::question::{NewQuestion, Question, QuestionId, UpdateQuestion};
use tracing::error;

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't connect to DB: {}", e),
        };

        Store {
            connection: db_pool,
        }
    }

    pub async fn get_questions(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Question>, error::Error> {
        match sqlx::query("SELECT * FROM questions LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(e) => {
                error!("{}", e);
                Err(error::Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn add_question(&self, new_question: NewQuestion) -> Result<Question, error::Error> {
        match sqlx::query(
            "INSERT INTO questions (title, content, tags) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(new_question.title)
        .bind(new_question.content)
        .bind(new_question.tags)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => {
                error!("{}", e);
                Err(error::Error::DatabaseQueryError(e))
            }
        }
    }

    /** Updates a question based on input JSON

    Will only update fields that are not null so that no value is overriden accidentally
    */
    pub async fn update_question(
        &self,
        question: UpdateQuestion,
        question_id: i32,
    ) -> Result<Question, error::Error> {
        match sqlx::query(
            "
        UPDATE questions
        SET 
            title=coalesce($1, title),
            content=coalesce($2, content),
            tags=coalesce($3, tags)
        WHERE id=$4
        RETURNING *",
        )
        .bind(question.title)
        .bind(question.content)
        .bind(question.tags)
        .bind(question_id)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => {
                error!("{}", e);
                Err(error::Error::DatabaseQueryError(e))
            }
        }
    }
}
