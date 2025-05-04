use crate::{
    db::models::post::{CreatePostDbInput, DbPost},
    domain::models::post::{CreatePostRequest, Post, PostBody, PostTitle},
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
