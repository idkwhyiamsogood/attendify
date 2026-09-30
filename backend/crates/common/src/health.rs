use axum::Json;
use axum::routing::get;
use axum::{Router, http::StatusCode};
use serde_json::json;

/// `/health` — общий для всех сервисов liveness-эндпоинт.
pub fn router() -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::OK, Json(json!({ "status": "ok" })))
}
