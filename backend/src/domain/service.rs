use async_trait::async_trait;
use thiserror::Error;

use crate::ids::PostId;

use super::{
    models::post::{CreatePostRequest, Post},
    repository::RepositoryError,
};

#[async_trait]
pub trait Service: Send + Sync + Clone + 'static {
    async fn create_post(&self, input: &CreatePostRequest) -> Result<Post, ServiceError>;

    async fn get_all_posts(&self) -> Result<Vec<Post>, ServiceError>;

    async fn get_posts_by_id(&self, id: PostId) -> Result<Post, ServiceError>;
}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error(transparent)]
    RepositoryError(RepositoryError),
}
