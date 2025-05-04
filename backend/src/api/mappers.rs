use crate::domain::models::post::{
    CreatePostRequest as DomainCreatePostRequest, Post, PostBody, PostTitle,
};

use super::post::{CreatePostRequest, PostResponse};

impl TryFrom<CreatePostRequest> for DomainCreatePostRequest {
    type Error = anyhow::Error;

    fn try_from(CreatePostRequest { title, body }: CreatePostRequest) -> Result<Self, Self::Error> {
        let title = PostTitle::try_new(&title)?;
        let body = PostBody::try_new(&body)?;
        Ok(Self::new(title, body))
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
