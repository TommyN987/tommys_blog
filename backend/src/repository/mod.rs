use async_trait::async_trait;
use tracing::{error, instrument};

use crate::{
    db::{postgres::Postgres, query},
    domain::{
        models::post::{CreatePostRequest, Post},
        repository::{CreatePostError, GetPostError, Repository, RepositoryError},
    },
    ids::PostId,
};

pub mod mappers;

#[async_trait]
impl Repository for Postgres {
    #[instrument(name = "repository_create_post", skip(self, input), err)]
    async fn create_post(&self, input: &CreatePostRequest) -> Result<Post, RepositoryError> {
        let db_input = input.into();

        match query::post::create_post(self.pool(), db_input).await {
            Ok(db_post) => {
                let post: Post = db_post.into();
                Ok(post)
            }
            Err(err) => {
                error!(?err, "Failed to create post in database");
                Err(RepositoryError::CreatePostError(CreatePostError::from((
                    err,
                    input.title(),
                ))))
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
    async fn get_post_by_id(&self, post_id: PostId) -> Result<Post, RepositoryError> {
        match query::post::get_post_by_id(self.pool(), post_id).await {
            Ok(db_post) => Ok(db_post.into()),
            Err(err) => {
                error!(?err, "Failed to get post with id {post_id} from database");
                Err(RepositoryError::GetPostError(GetPostError::from((
                    err, post_id,
                ))))
            }
        }
    }
}
