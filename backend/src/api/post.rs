use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument};
use uuid::Uuid;

use crate::domain::repository::RepositoryError;
use crate::domain::service::ServiceError;
use crate::domain::{
    models::post::CreatePostRequest as DomainCreatePostRequest, repository::CreatePostError,
    service::Service,
};
use crate::server::AppState;

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

pub fn routes<S: Service>() -> Router<AppState<S>> {
    Router::new().route("/posts", post(create_post::<S>))
}

#[instrument(name = "create_post_handler", skip(state), fields(title = %payload.title))]
async fn create_post<S: Service>(
    State(state): State<AppState<S>>,
    Json(payload): Json<CreatePostRequest>,
) -> impl IntoResponse {
    debug!("Received create post request");

    let domain_req = match DomainCreatePostRequest::try_from(payload) {
        Ok(req) => {
            debug!("Converted API request to domain request");
            req
        }
        Err(err) => {
            error!(?err, "Failed to convert API request to domain request");
            return (StatusCode::BAD_REQUEST, Json("Invalid request data")).into_response();
        }
    };

    match state.service.create_post(&domain_req).await {
        Ok(post) => {
            info!(post_id = %post.id(), "Successfully created post");
            (StatusCode::CREATED, Json(PostResponse::from(post))).into_response()
        }
        Err(ServiceError::RepositoryError(RepositoryError::CreatePostError(
            CreatePostError::Duplicate { title },
        ))) => {
            error!(%title, "Duplicate post title");
            let error_msg = format!("Post with title '{}' already exists", title);
            (StatusCode::UNPROCESSABLE_ENTITY, Json(error_msg)).into_response()
        }
        Err(err) => {
            error!(?err, "Unknown error creating post");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("An unexpected error occurred"),
            )
                .into_response()
        }
    }
}
