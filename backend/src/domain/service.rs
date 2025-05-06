use async_trait::async_trait;

use super::{
    models::post::{CreatePostRequest, Post},
    repository::RepositoryError,
};

#[async_trait]
pub trait Service: Send + Sync + Clone + 'static {
    async fn create_post(&self, input: &CreatePostRequest) -> Result<Post, RepositoryError>;
}
