use axum::{Json, Router, routing::get};
use serde::Serialize;
use tracing::{info, instrument};

use crate::domain::repository::Repository;
use crate::server::AppState;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

pub fn routes<R: Repository>() -> Router<AppState<R>> {
    Router::new().route("/health", get(health_check))
}

#[instrument(name = "health_check_handler")]
async fn health_check() -> Json<HealthResponse> {
    info!("✅ Health check called");
    Json(HealthResponse { status: "ok" })
}
