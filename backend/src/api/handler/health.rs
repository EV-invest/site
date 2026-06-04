use axum::{Json, http::StatusCode};
use serde_json::{Value, json};

/// Liveness probe — `GET /health`.
pub async fn health() -> (StatusCode, Json<Value>) {
	(StatusCode::OK, Json(json!({ "status": "ok" })))
}
