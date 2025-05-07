use crate::domain::{
    models::post::{CreatePostRequest as DomainCreatePostRequest, Post, PostBody, PostTitle},
    service::ServiceError,
};

use super::{
    post::{CreatePostRequest, CreatePostRequestError, PostResponse},
    responses::ApiError,
};

impl TryFrom<CreatePostRequest> for DomainCreatePostRequest {
    type Error = ApiError;

    fn try_from(CreatePostRequest { title, body }: CreatePostRequest) -> Result<Self, Self::Error> {
        let title = PostTitle::try_new(&title).map_err(CreatePostRequestError::from)?;
        let body = PostBody::try_new(&body).map_err(CreatePostRequestError::from)?;
        Ok(Self::new(title, body))
    }
}

impl From<ServiceError> for ApiError {
    fn from(service_error: ServiceError) -> Self {
        use crate::domain::{
            repository::{
                CreatePostError::*,
                RepositoryError::{CreatePostError, Unknown as RepoUnknown},
            },
            service::ServiceError::*,
        };

        match service_error {
            RepositoryError(repo_error) => match repo_error {
                CreatePostError(error) => match error {
                    Duplicate { title } => ApiError::Conflict(title.to_string()),
                    Unknown(e) => e.into(),
                },
                RepoUnknown(e) => e.into(),
            },
        }
    }
}

impl From<Post> for PostResponse {
    fn from(value: Post) -> Self {
        Self {
            id: value.id(),
            title: value.title().to_string(),
            body: value.body().to_string(),
            created_at: value.created_at(),
        }
    }
}
