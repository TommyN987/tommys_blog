use sqlx::{PgPool, Row, error::Error as SqlxError, postgres::PgRow};

use crate::{
    db::models::post::{CreatePostDbInput, DbPost, UpdatePostDbInput},
    ids::PostId,
};

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
    let id = PostId::new();

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

pub async fn get_post_by_id(pool: &PgPool, id: PostId) -> Result<DbPost, SqlxError> {
    let query_result = sqlx::query(
        r#"
            SELECT * FROM posts
            WHERE id=($1)
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    DbPost::try_from(query_result)
}

pub async fn get_post_by_title(pool: &PgPool, title: &str) -> Result<DbPost, SqlxError> {
    let query_result = sqlx::query(
        r#"
            SELECT * FROM posts
            WHERE title=($1)
        "#,
    )
    .bind(title)
    .fetch_one(pool)
    .await?;

    DbPost::try_from(query_result)
}

pub async fn update_post(
    pool: &PgPool,
    id: PostId,
    UpdatePostDbInput { title, body }: UpdatePostDbInput,
) -> Result<DbPost, SqlxError> {
    let query_result = sqlx::query(
        r#"
            UPDATE posts
            SET 
                title = COALESCE($1, title),
                body = COALESCE($2, body)
            WHERE id = $3
            RETURNING id, title, body, created_at
        "#,
    )
    .bind(title)
    .bind(body)
    .bind(id)
    .fetch_one(pool)
    .await?;

    DbPost::try_from(query_result)
}

pub async fn delete_post(pool: &PgPool, id: PostId) -> Result<(), SqlxError> {
    sqlx::query(
        r#"
            DELETE FROM posts WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}
