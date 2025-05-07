use sqlx::{PgPool, Row, error::Error as SqlxError, postgres::PgRow};
use uuid::Uuid;

use crate::db::models::post::{CreatePostDbInput, DbPost};

impl TryFrom<PgRow> for DbPost {
    type Error = SqlxError;

    fn try_from(row: PgRow) -> Result<Self, Self::Error> {
        Ok(DbPost {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            body: row.try_get("body")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

pub async fn create_post(pool: &PgPool, input: CreatePostDbInput) -> Result<DbPost, SqlxError> {
    let id = Uuid::new_v4();

    let query_result = sqlx::query(
        r#"
            INSERT INTO posts (id, title, body)
            VALUES ($1, $2, $3)
            RETURNING id, title, body, created_at
        "#,
    )
    .bind(id)
    .bind(input.title())
    .bind(input.body())
    .fetch_one(pool)
    .await;

    DbPost::try_from(query_result?)
}

pub async fn get_all_posts(pool: &PgPool) -> Result<Vec<DbPost>, SqlxError> {
    let query_results = sqlx::query(
        r#"
            SELECT * FROM posts
        "#,
    )
    .fetch_all(pool)
    .await?;

    query_results
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<Vec<DbPost>, SqlxError>>()
}
