use anyhow::anyhow;
use chrono::{DateTime, Utc};
use sqlx::{Error as SqlxError, error::ErrorKind};
use thiserror::Error as ThisError;
use uuid::Uuid;

pub struct DbPost {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

pub struct CreatePostDbInput {
    title: String,
    body: String,
}

impl CreatePostDbInput {
    pub fn new(title: String, body: String) -> Self {
        Self { title, body }
    }

    pub(crate) fn title(&self) -> &str {
        &self.title
    }

    pub(crate) fn body(&self) -> &str {
        &self.body
    }
}

#[derive(Debug, ThisError)]
pub enum CreatePostError {
    #[error("Blog post with title {title} already exists.")]
    Duplicate { title: String },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<(SqlxError, &str)> for CreatePostError {
    fn from((error, title): (SqlxError, &str)) -> Self {
        match &error {
            SqlxError::Database(e) => match e.kind() {
                ErrorKind::UniqueViolation => Self::Duplicate {
                    title: title.to_string(),
                },
                // TODO: Cover other variants
                _ => Self::Unknown(anyhow!(error)),
            },
            _ => Self::Unknown(anyhow!(error)),
        }
    }
}
