use axum::routing::get;
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{error, instrument};
use uuid::Uuid;

use crate::domain::models::post::{PostBodyEmptyError, PostTitleEmptyError};
use crate::domain::{models::post::CreatePostRequest as DomainCreatePostRequest, service::Service};
use crate::server::AppState;

use super::responses::{ApiError, ApiResult, ApiSuccess};

#[derive(Debug, Deserialize)]
pub(super) struct CreatePostRequest {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, Error)]
pub(super) enum CreatePostRequestError {
    #[error(transparent)]
    Title(#[from] PostTitleEmptyError),
    #[error(transparent)]
    Body(#[from] PostBodyEmptyError),
}

impl From<CreatePostRequestError> for ApiError {
    fn from(e: CreatePostRequestError) -> Self {
        error!(?e, "Failed to convert API request to domain request");
        Self::UnprossableEntity(e.to_string())
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub(super) struct PostResponse {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub(super) struct BulkPostResponse {
    pub data: Vec<PostResponse>,
}

pub fn routes<S: Service>() -> Router<AppState<S>> {
    Router::new()
        .route("/posts", post(create_post::<S>))
        .route("/posts", get(get_posts::<S>))
}

#[instrument(name = "create_post_handler", skip(state), fields(title = %payload.title))]
async fn create_post<S: Service>(
    State(state): State<AppState<S>>,
    Json(payload): Json<CreatePostRequest>,
) -> ApiResult<PostResponse> {
    let domain_req = DomainCreatePostRequest::try_from(payload)?;

    state
        .service()
        .create_post(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|post| ApiSuccess::new(StatusCode::CREATED, post.into()))
}

async fn get_posts<S: Service>(State(state): State<AppState<S>>) -> ApiResult<BulkPostResponse> {
    let data: Vec<PostResponse> = state
        .service()
        .get_all_posts()
        .await
        .map_err(ApiError::from)?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(ApiSuccess::new(StatusCode::OK, BulkPostResponse { data }))
}
