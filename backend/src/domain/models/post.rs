use std::fmt::Display;

use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Post {
    id: Uuid,
    title: PostTitle,
    body: PostBody,
    created_at: DateTime<Utc>,
}

impl Post {
    pub fn new(id: Uuid, title: PostTitle, body: PostBody, created_at: DateTime<Utc>) -> Self {
        Self {
            id,
            title,
            body,
            created_at,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id.clone()
    }

    pub fn title(&self) -> PostTitle {
        self.title.clone()
    }

    pub fn body(&self) -> PostBody {
        self.body.clone()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreatePostRequest {
    title: PostTitle,
    body: PostBody,
}

impl CreatePostRequest {
    pub fn new(title: PostTitle, body: PostBody) -> Self {
        Self { title, body }
    }
    pub fn title(&self) -> PostTitle {
        self.title.clone()
    }

    pub fn body(&self) -> PostBody {
        self.body.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PostTitle(String);

#[derive(Clone, Debug, Error)]
#[error("Blog post title cannot be empty")]
pub struct PostTitleEmptyError;

impl PostTitle {
    pub fn try_new(raw: &str) -> Result<Self, PostTitleEmptyError> {
        validate_non_empty(raw).map(Self).ok_or(PostTitleEmptyError)
    }

    pub fn new(input: &str) -> Self {
        Self(input.to_string())
    }
}

impl Display for PostTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PostBody(String);

impl Display for PostBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Error)]
#[error("Blog post body cannot be empty")]
pub struct PostBodyEmptyError;

impl PostBody {
    pub fn try_new(raw: &str) -> Result<Self, PostBodyEmptyError> {
        validate_non_empty(raw).map(Self).ok_or(PostBodyEmptyError)
    }

    pub fn new(input: &str) -> Self {
        Self(input.to_string())
    }
}

fn validate_non_empty(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}
