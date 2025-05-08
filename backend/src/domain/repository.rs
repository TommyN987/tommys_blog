use async_trait::async_trait;
use thiserror::Error;

use crate::ids::PostId;

use super::models::post::{CreatePostRequest, Post, PostTitle, UpdatePostRequest};

#[async_trait]
pub trait Repository: Send + Sync + Clone + 'static {
    async fn create_post(&self, input: &CreatePostRequest) -> Result<Post, CreatePostError>;

    async fn get_all_posts(&self) -> Result<Vec<Post>, RepositoryError>;

    async fn get_post_by_id(&self, post_id: PostId) -> Result<Post, GetPostError>;

    async fn update_post(
        &self,
        post_id: PostId,
        input: &UpdatePostRequest,
    ) -> Result<Post, UpdatePostError>;
}

pub trait IntoRepositoryError {
    fn into_repository_error(self) -> RepositoryError;
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error(transparent)]
    CreatePostError(CreatePostError),
    #[error(transparent)]
    GetPostError(GetPostError),
    #[error(transparent)]
    UpdatePostError(UpdatePostError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum GetPostError {
    #[error("Could not find blog post with id {id}.")]
    PostNotFound { id: PostId },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum CreatePostError {
    #[error("Blog post with title {title} already exists.")]
    Duplicate { title: PostTitle },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum UpdatePostError {
    #[error("Could not find blog post with id {id}.")]
    PostNotFound { id: PostId },
    #[error("Blog post with title {title} already exists.")]
    Duplicate { title: PostTitle },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl IntoRepositoryError for CreatePostError {
    fn into_repository_error(self) -> RepositoryError {
        RepositoryError::CreatePostError(self)
    }
}

impl IntoRepositoryError for GetPostError {
    fn into_repository_error(self) -> RepositoryError {
        RepositoryError::GetPostError(self)
    }
}

impl IntoRepositoryError for UpdatePostError {
    fn into_repository_error(self) -> RepositoryError {
        RepositoryError::UpdatePostError(self)
    }
}
