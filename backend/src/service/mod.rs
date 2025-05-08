use async_trait::async_trait;

use crate::{
    domain::{
        models::post::{CreatePostRequest, Post, UpdatePostRequest},
        repository::{IntoRepositoryError, Repository},
        service::{Service, ServiceError},
    },
    ids::PostId,
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
        Ok(self
            .repo
            .create_post(input)
            .await
            .map_err(IntoRepositoryError::into_repository_error)?)
    }

    async fn get_all_posts(&self) -> Result<Vec<Post>, ServiceError> {
        Ok(self.repo.get_all_posts().await?)
    }

    async fn get_posts_by_id(&self, id: PostId) -> Result<Post, ServiceError> {
        Ok(self
            .repo
            .get_post_by_id(id)
            .await
            .map_err(IntoRepositoryError::into_repository_error)?)
    }

    async fn update_post(
        &self,
        post_id: PostId,
        input: &UpdatePostRequest,
    ) -> Result<Post, ServiceError> {
        Ok(self
            .repo
            .update_post(post_id, input)
            .await
            .map_err(IntoRepositoryError::into_repository_error)?)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use mockall::predicate::*;
    use mockall::*;

    use crate::domain::models::post::{PostBody, PostTitle};
    use crate::domain::repository::{
        CreatePostError, GetPostError, RepositoryError, UpdatePostError,
    };

    use super::*;

    mock! {
        Repository {}
        impl Clone for Repository {
            fn clone(&self) -> Self;
        }

        #[async_trait]
        impl Repository for Repository {
            async fn create_post(&self, input: &CreatePostRequest) -> Result<Post, CreatePostError>;
            async fn get_all_posts(&self) -> Result<Vec<Post>, RepositoryError>;
            async fn get_post_by_id(&self, post_id: PostId) -> Result<Post, GetPostError>;
            async fn update_post(
                &self,
                post_id: PostId,
                input: &UpdatePostRequest,
            ) -> Result<Post, UpdatePostError>;
        }
    }

    #[tokio::test]
    async fn test_blog_service_create_post_success() {
        let mut mock_repo = MockRepository::new();

        let title = PostTitle::new("Test title");
        let body = PostBody::new("Test body");
        let create_req = CreatePostRequest::new(title.clone(), body.clone());

        let expected_post = Post::new(PostId::new(), title.clone(), body.clone(), Utc::now());

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
