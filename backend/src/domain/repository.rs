use async_trait::async_trait;
use thiserror::Error;

use super::models::post::{CreatePostRequest, Post, PostTitle};

#[async_trait]
pub trait Repository: Send + Sync + Clone + 'static {
    async fn create_post(&self, input: &CreatePostRequest) -> Result<Post, CreatePostError>;
}

#[derive(Debug, Error)]
pub enum CreatePostError {
    #[error("Blog post with title {title} already exists.")]
    Duplicate { title: PostTitle },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
