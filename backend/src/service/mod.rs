use async_trait::async_trait;

use crate::domain::{
    models::post::{CreatePostRequest, Post},
    repository::{Repository, RepositoryError},
    service::Service,
};

#[derive(Debug, Clone)]
pub struct BlogService<R: Repository> {
    repo: R,
}

impl<R> BlogService<R>
where
    R: Repository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R> Service for BlogService<R>
where
    R: Repository,
{
    async fn create_post(&self, input: &CreatePostRequest) -> Result<Post, RepositoryError> {
        self.repo.create_post(input).await
    }
}
