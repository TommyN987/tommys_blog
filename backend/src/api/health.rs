use axum::{Json, Router, routing::get};
use serde::Serialize;
use tracing::{info, instrument};

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

pub fn routes() -> Router {
    Router::new().route("/health", get(health_check))
}

#[instrument(name = "health_check_handler")]
async fn health_check() -> Json<HealthResponse> {
    info!("✅ Health check called");
    Json(HealthResponse { status: "ok" })
}
