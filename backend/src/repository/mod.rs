use async_trait::async_trait;
use tracing::{debug, error, instrument};

use crate::{
    db::{postgres::Postgres, query},
    domain::{
        models::post::{CreatePostRequest, Post},
        repository::{CreatePostError, Repository},
    },
};

pub mod mappers;

#[async_trait]
impl Repository for Postgres {
    #[instrument(name = "repository_create_post", skip(self, input), err)]
    async fn create_post(&self, input: &CreatePostRequest) -> Result<Post, CreatePostError> {
        debug!("Converting domain model to DB input");
        let db_input = input.into();

        debug!("Executing database query");
        match query::post::create_post(self.pool(), db_input).await {
            Ok(db_post) => {
                debug!("Successfully inserted post in database");
                let post: Post = db_post.into();
                debug!(post_id = %post.id(), "Converted DB post to domain model");
                Ok(post)
            }
            Err(err) => {
                error!(?err, "Failed to create post in database");
                Err(CreatePostError::Unknown(err))
            }
        }
    }
}
