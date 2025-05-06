use async_trait::async_trait;

use crate::domain::{
    models::post::{CreatePostRequest, Post},
    repository::Repository,
    service::{Service, ServiceError},
};

pub mod mappers;

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
    async fn create_post(&self, input: &CreatePostRequest) -> Result<Post, ServiceError> {
        Ok(self.repo.create_post(input).await?)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use mockall::predicate::*;
    use mockall::*;
    use uuid::Uuid;

    use crate::domain::models::post::{PostBody, PostTitle};
    use crate::domain::repository::RepositoryError;

    use super::*;

    mock! {
        Repository {}
        impl Clone for Repository {
            fn clone(&self) -> Self;
        }

        #[async_trait]
        impl Repository for Repository {
            async fn create_post(&self, input: &CreatePostRequest) -> Result<Post, RepositoryError>;
        }
    }

    #[tokio::test]
    async fn test_blog_service_create_post_success() {
        let mut mock_repo = MockRepository::new();

        let title = PostTitle::new("Test title");
        let body = PostBody::new("Test body");
        let create_req = CreatePostRequest::new(title.clone(), body.clone());

        let expected_post = Post::new(Uuid::new_v4(), title.clone(), body.clone(), Utc::now());

        mock_repo
            .expect_create_post()
            .with(predicate::always())
            .returning(move |_| Ok(expected_post.clone()));

        let service = BlogService::new(mock_repo);

        let result = service.create_post(&create_req).await;

        assert!(result.is_ok());

        let post = result.unwrap();

        assert_eq!(post.title(), title);
        assert_eq!(post.body(), body);
    }
}
