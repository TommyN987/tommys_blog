use async_trait::async_trait;

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
    async fn create_post(&self, input: &CreatePostRequest) -> Result<Post, CreatePostError> {
        Ok(query::post::create_post(self.pool(), input.into())
            .await?
            .into())
    }
}
