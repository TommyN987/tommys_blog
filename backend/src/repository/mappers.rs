use anyhow::anyhow;
use sqlx::{Error as SqlxError, error::ErrorKind};

use crate::{
    db::models::post::{CreatePostDbInput, DbPost},
    domain::{
        models::post::{CreatePostRequest, Post, PostBody, PostTitle},
        repository::{CreatePostError, GetPostError},
    },
    ids::PostId,
};

impl From<&CreatePostRequest> for CreatePostDbInput {
    fn from(value: &CreatePostRequest) -> Self {
        let title = value.title().to_string();
        let body = value.body().to_string();

        Self::new(title, body)
    }
}

impl From<DbPost> for Post {
    fn from(
        DbPost {
            id,
            title,
            body,
            created_at,
        }: DbPost,
    ) -> Self {
        let title = PostTitle::new(&title);
        let body = PostBody::new(&body);

        Self::new(id, title, body, created_at)
    }
}

impl From<(SqlxError, PostTitle)> for CreatePostError {
    fn from((error, title): (SqlxError, PostTitle)) -> Self {
        match &error {
            SqlxError::Database(e) => match e.kind() {
                ErrorKind::UniqueViolation => Self::Duplicate { title },
                // TODO: Cover other variants
                _ => Self::Unknown(anyhow!(error)),
            },
            _ => Self::Unknown(anyhow!(error)),
        }
    }
}

impl From<(SqlxError, PostId)> for GetPostError {
    fn from((error, id): (SqlxError, PostId)) -> Self {
        match &error {
            SqlxError::RowNotFound => Self::PostNotFound { id },
            _ => Self::Unknown(anyhow!(error)),
        }
    }
}
