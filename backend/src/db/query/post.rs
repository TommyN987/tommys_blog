use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use tracing::{debug, error, instrument};
use uuid::Uuid;

use crate::db::models::post::{CreatePostDbInput, DbPost};

#[instrument(name = "db_create_post", skip(pool, input), err)]
pub async fn create_post(pool: &PgPool, input: CreatePostDbInput) -> Result<DbPost, anyhow::Error> {
    let id = Uuid::new_v4();
    debug!(post_id = %id, "Generated new post ID");

    debug!(post_id = %id, "Executing INSERT query");
    // Using query instead of query_as to manually construct the DbPost
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

    match query_result {
        Ok(row) => {
            debug!(post_id = %id, "Successfully executed INSERT query");

            debug!(post_id = %id, "Extracting created_at from row");
            let created_at_result: Result<DateTime<Utc>, _> = row.try_get("created_at");

            match created_at_result {
                Ok(created_at) => {
                    debug!(post_id = %id, created_at = %created_at, "Successfully extracted created_at");

                    let db_post = DbPost {
                        id: row.get("id"),
                        title: row.get("title"),
                        body: row.get("body"),
                        created_at,
                    };

                    debug!(post_id = %id, "Successfully constructed DbPost");
                    Ok(db_post)
                }
                Err(err) => {
                    error!(?err, post_id = %id, "Failed to extract created_at from row");
                    Err(anyhow::anyhow!("Failed to extract created_at: {}", err))
                }
            }
        }
        Err(err) => {
            error!(?err, post_id = %id, "Failed to execute INSERT query");
            Err(anyhow::anyhow!("Failed to execute INSERT query: {}", err))
        }
    }
}
