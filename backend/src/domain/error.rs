use thiserror::Error;
use uuid::Uuid;

/// Errors arising from the domain / application core.
///
/// Driving adapters translate these into their own representation at the
/// boundary (the HTTP layer maps them to status codes — see
/// `crate::api::error::ApiError`).
#[derive(Debug, Error)]
pub enum DomainError {
	#[error("{entity} not found: {id}")]
	NotFound { entity: &'static str, id: Uuid },
	#[error("conflict: {0}")]
	Conflict(String),
	#[error("validation failed: {0}")]
	Validation(String),
	/// Unexpected failure from a driven adapter (e.g. the database). Carries a
	/// description for logging only — it is never surfaced verbatim to clients,
	/// and an infrastructure failure must never be mapped to `Validation`.
	#[error("repository error: {0}")]
	Repository(String),
}
