use axum::{Extension, Json, Router, http::StatusCode, response::IntoResponse, routing::post};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::db::postgres::Postgres;
use crate::domain::{
    models::post::CreatePostRequest as DomainCreatePostRequest,
    repository::{CreatePostError, Repository},
};

#[derive(Debug, Deserialize)]
pub(super) struct CreatePostRequest {
    pub title: String,
    pub body: String,
}

#[derive(Serialize)]
pub(super) struct PostResponse {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

pub fn routes() -> Router {
    Router::new().route("/posts", post(create_post::<Postgres>))
}

#[instrument(name = "create_post_handler", skip(repo))]
async fn create_post<R: Repository>(
    Extension(repo): Extension<R>,
    Json(payload): Json<CreatePostRequest>,
) -> impl IntoResponse {
    // Convert API model to domain model
    let domain_req = match DomainCreatePostRequest::try_from(payload) {
        Ok(req) => req,
        Err(_) => return (StatusCode::BAD_REQUEST, Json("Invalid request data")).into_response(),
    };

    // Create post in repository
    match repo.create_post(&domain_req).await {
        Ok(post) => (StatusCode::CREATED, Json(PostResponse::from(post))).into_response(),
        Err(CreatePostError::Duplicate { title }) => {
            let error_msg = format!("Post with title '{}' already exists", title);
            (StatusCode::CONFLICT, Json(error_msg)).into_response()
        }
        Err(CreatePostError::Unknown(_)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("An unexpected error occurred"),
        )
            .into_response(),
    }
}
