use chrono::{DateTime, Utc};

use crate::ids::PostId;

pub struct DbPost {
    pub id: PostId,
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
