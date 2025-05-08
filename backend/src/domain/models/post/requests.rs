use super::model::{PostBody, PostTitle};

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
pub struct UpdatePostRequest {
    title: Option<PostTitle>,
    body: Option<PostBody>,
}

impl UpdatePostRequest {
    pub fn new(title: Option<PostTitle>, body: Option<PostBody>) -> Self {
        Self { title, body }
    }

    pub fn title(&self) -> Option<PostTitle> {
        self.title.clone()
    }

    pub fn body(&self) -> Option<PostBody> {
        self.body.clone()
    }
}
