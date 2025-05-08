use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

pub(super) type ApiResult<T> = Result<ApiSuccess<T>, ApiError>;

#[derive(Debug, Clone)]
pub(super) struct ApiSuccess<T: Serialize + PartialEq>(StatusCode, Json<ApiResponseBody<T>>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct ApiResponseBody<T: Serialize + PartialEq>(T);

impl<T> PartialEq for ApiSuccess<T>
where
    T: Serialize + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1.0 == other.1.0
    }
}

impl<T: Serialize + PartialEq> ApiSuccess<T> {
    pub(super) fn new(status: StatusCode, data: T) -> Self {
        ApiSuccess(status, Json(ApiResponseBody::new(data)))
    }
}

impl<T: Serialize + PartialEq> IntoResponse for ApiSuccess<T> {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}

impl<T: Serialize + PartialEq> ApiResponseBody<T> {
    pub fn new(data: T) -> Self {
        Self(data)
    }
}

impl ApiResponseBody<ApiErrorData> {
    pub fn new_error(message: String) -> Self {
        Self(ApiErrorData { message })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    Conflict(String),
    NotFound(String),
    UnprocessableEntity(String),
    InternalServerError(String),
}

impl ApiError {
    fn generate_response_input(status_code: StatusCode, message: String) -> Response {
        (status_code, Json(ApiResponseBody::new_error(message))).into_response()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e.to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        use ApiError::*;

        match self {
            InternalServerError(e) => {
                tracing::error!("{}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponseBody::new_error(
                        "Internal server error".to_string(),
                    )),
                )
                    .into_response()
            }
            NotFound(message) => Self::generate_response_input(StatusCode::NOT_FOUND, message),
            UnprocessableEntity(message) => {
                Self::generate_response_input(StatusCode::UNPROCESSABLE_ENTITY, message)
            }
            Conflict(message) => Self::generate_response_input(StatusCode::CONFLICT, message),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiErrorData {
    pub message: String,
}
