use std::fmt::Display;

use chrono::{DateTime, Utc};
use thiserror::Error;

use crate::ids::PostId;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Post {
    id: PostId,
    title: PostTitle,
    body: PostBody,
    created_at: DateTime<Utc>,
}

impl Post {
    pub fn new(id: PostId, title: PostTitle, body: PostBody, created_at: DateTime<Utc>) -> Self {
        Self {
            id,
            title,
            body,
            created_at,
        }
    }

    pub fn id(&self) -> PostId {
        self.id
    }

    pub fn title(&self) -> PostTitle {
        self.title.clone()
    }

    pub fn body(&self) -> PostBody {
        self.body.clone()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_title_validation_with_valid_input() {
        // Given a non-empty title
        let title = "My Post Title";

        // When we create a PostTitle
        let result = PostTitle::try_new(title);

        // Then it should be successful
        assert!(result.is_ok());
        let post_title = result.unwrap();
        assert_eq!(post_title.to_string(), title);
    }

    #[test]
    fn test_post_title_validation_with_empty_input() {
        // Given an empty title
        let title = "";

        // When we create a PostTitle
        let result = PostTitle::try_new(title);

        // Then it should fail
        assert!(result.is_err());
    }

    #[test]
    fn test_post_title_validation_with_whitespace_input() {
        // Given a title with only whitespace
        let title = "   \t\n  ";

        // When we create a PostTitle
        let result = PostTitle::try_new(title);

        // Then it should fail
        assert!(result.is_err());
    }

    #[test]
    fn test_post_body_validation_with_valid_input() {
        // Given a non-empty body
        let body = "This is the post content.";

        // When we create a PostBody
        let result = PostBody::try_new(body);

        // Then it should be successful
        assert!(result.is_ok());
        let post_body = result.unwrap();
        assert_eq!(post_body.to_string(), body);
    }

    #[test]
    fn test_post_body_validation_with_empty_input() {
        // Given an empty body
        let body = "";

        // When we create a PostBody
        let result = PostBody::try_new(body);

        // Then it should fail
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_non_empty_function() {
        assert_eq!(validate_non_empty("test"), Some("test".to_string()));
        assert_eq!(validate_non_empty(" test "), Some("test".to_string()));
        assert_eq!(validate_non_empty(""), None);
        assert_eq!(validate_non_empty("   "), None);
    }
}
