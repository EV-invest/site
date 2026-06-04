use thiserror::Error;

/// Errors arising from the domain / application core.
///
/// Driving adapters translate these into their own representation at the
/// boundary (e.g. the HTTP layer maps them to status codes — see
/// `crate::api::error::ApiError`).
#[derive(Debug, Error)]
pub enum DomainError {
	#[error("entity not found")]
	NotFound,
	#[error("conflict: {0}")]
	Conflict(String),
	#[error("validation failed: {0}")]
	Validation(String),
	/// Unexpected failure bubbling up from a driven adapter (e.g. the database).
	#[error("repository error: {0}")]
	Repository(#[from] anyhow::Error),
}
