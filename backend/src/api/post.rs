use axum::routing::{get, patch};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::post,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{error, instrument};

use crate::domain::models::post::{PostBodyEmptyError, PostTitleEmptyError};
use crate::domain::{
    models::post::{
        CreatePostRequest as DomainCreatePostRequest, UpdatePostRequest as DomainUpdatePostRequest,
    },
    service::Service,
};
use crate::ids::PostId;
use crate::server::AppState;

use super::responses::{ApiError, ApiResult, ApiSuccess};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Error)]
pub(super) enum CreatePostRequestError {
    #[error(transparent)]
    Title(#[from] PostTitleEmptyError),
    #[error(transparent)]
    Body(#[from] PostBodyEmptyError),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PostResponse {
    pub id: PostId,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BulkPostResponse {
    pub data: Vec<PostResponse>,
}

pub fn routes<S: Service>() -> Router<AppState<S>> {
    Router::new()
        .route("/posts", post(create_post::<S>))
        .route("/posts", get(get_posts::<S>))
        .route("/posts/{post_id}", get(get_post_by_id::<S>))
        .route("/posts/{post_id}", patch(update_post::<S>))
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

#[instrument(name = "get_posts", skip(state))]
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

async fn get_post_by_id<S: Service>(
    State(state): State<AppState<S>>,
    Path(post_id): Path<PostId>,
) -> ApiResult<PostResponse> {
    state
        .service()
        .get_posts_by_id(post_id)
        .await
        .map_err(ApiError::from)
        .map(|post| ApiSuccess::new(StatusCode::OK, post.into()))
}

async fn update_post<S: Service>(
    State(state): State<AppState<S>>,
    Path(post_id): Path<PostId>,
    Json(payload): Json<UpdatePostRequest>,
) -> ApiResult<PostResponse> {
    let domain_req = DomainUpdatePostRequest::try_from(payload)?;

    state
        .service()
        .update_post(post_id, &domain_req)
        .await
        .map_err(ApiError::from)
        .map(|post| ApiSuccess::new(StatusCode::OK, post.into()))
}
