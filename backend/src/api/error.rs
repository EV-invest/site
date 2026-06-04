use axum::{
	Json,
	http::StatusCode,
	response::{IntoResponse, Response},
};
use serde_json::json;

use crate::domain::error::DomainError;

/// HTTP-facing error. Wraps [`DomainError`] and maps it to a status code plus a
/// JSON body at the transport boundary. Handlers return `Result<_, ApiError>`
/// and rely on `?` to convert domain errors via the `From` impl below.
pub struct ApiError(pub DomainError);

impl From<DomainError> for ApiError {
	fn from(err: DomainError) -> Self {
		Self(err)
	}
}

impl IntoResponse for ApiError {
	fn into_response(self) -> Response {
		let (status, message) = match self.0 {
			DomainError::NotFound => (StatusCode::NOT_FOUND, "not found".to_string()),
			DomainError::Conflict(msg) => (StatusCode::CONFLICT, msg),
			DomainError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
			DomainError::Repository(err) => {
				// Internal details are logged, never leaked to the client.
				tracing::error!(error = %err, "internal repository error");
				(StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string())
			}
		};
		(status, Json(json!({ "error": message }))).into_response()
	}
}
