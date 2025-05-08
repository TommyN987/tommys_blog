use async_trait::async_trait;
use tracing::{error, instrument};

use crate::{
    db::{postgres::Postgres, query},
    domain::{
        models::post::{CreatePostRequest, Post, UpdatePostRequest},
        repository::{CreatePostError, GetPostError, Repository, RepositoryError, UpdatePostError},
    },
    ids::PostId,
};

pub mod mappers;

#[async_trait]
impl Repository for Postgres {
    #[instrument(name = "repository_create_post", skip(self, input), err)]
    async fn create_post(&self, input: &CreatePostRequest) -> Result<Post, CreatePostError> {
        let db_input = input.into();

        match query::post::create_post(self.pool(), db_input).await {
            Ok(db_post) => {
                let post: Post = db_post.into();
                Ok(post)
            }
            Err(err) => {
                error!(?err, "Failed to create post in database");
                Err(CreatePostError::from((err, input.title())))
            }
        }
    }

    #[instrument(name = "repository_get_all_posts", skip(self), err)]
    async fn get_all_posts(&self) -> Result<Vec<Post>, RepositoryError> {
        match query::post::get_all_posts(self.pool()).await {
            Ok(db_posts) => {
                let posts: Vec<Post> = db_posts.into_iter().map(Into::into).collect();
                Ok(posts)
            }
            Err(err) => {
                error!(?err, "Failed to get all posts from database");
                Err(RepositoryError::Unknown(err.into()))
            }
        }
    }

    #[instrument(name = "repository_get_post_by_id", skip(self, post_id), err)]
    async fn get_post_by_id(&self, post_id: PostId) -> Result<Post, GetPostError> {
        match query::post::get_post_by_id(self.pool(), post_id).await {
            Ok(db_post) => Ok(db_post.into()),
            Err(err) => {
                error!(?err, "Failed to get post with id {post_id} from database");
                Err(GetPostError::from((err, post_id)))
            }
        }
    }

    #[instrument(name = "repository_update_post", skip(self, post_id, input), err)]
    async fn update_post(
        &self,
        post_id: PostId,
        input: &UpdatePostRequest,
    ) -> Result<Post, UpdatePostError> {
        let db_input = input.into();

        if let Some(title) = input.title() {
            if query::post::get_post_by_title(self.pool(), title.to_string().as_str())
                .await
                .is_ok()
            {
                return Err(UpdatePostError::Duplicate { title });
            }
        }

        match query::post::update_post(self.pool(), post_id, db_input).await {
            Ok(db_post) => Ok(db_post.into()),
            Err(err) => Err(UpdatePostError::from((err, post_id))),
        }
    }
}
