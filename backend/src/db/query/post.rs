use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::db::models::post::{CreatePostDbInput, DbPost};

pub async fn create_post(pool: &PgPool, input: CreatePostDbInput) -> Result<DbPost, anyhow::Error> {
    let id = Uuid::new_v4();

    // Using query instead of query_as to manually construct the DbPost
    let row = sqlx::query(
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
    .await?;

    let created_at: DateTime<Utc> = row.get("created_at");

    Ok(DbPost {
        id: row.get("id"),
        title: row.get("title"),
        body: row.get("body"),
        created_at,
    })
}
