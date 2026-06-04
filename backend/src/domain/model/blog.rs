use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::error::DomainError;

/// Maximum number of characters allowed in a [`Title`].
const TITLE_MAX_LEN: usize = 200;

/// A validated blog title: trimmed, non-empty, at most [`TITLE_MAX_LEN`] chars.
/// Constructing one is proof the invariant holds.
#[derive(Debug, Clone)]
pub struct Title(String);

impl Title {
	/// The only constructor. Enforces the invariant, so a `Title` cannot exist
	/// in an invalid state.
	pub fn parse(raw: String) -> Result<Self, DomainError> {
		let trimmed = raw.trim();
		if trimmed.is_empty() {
			return Err(DomainError::Validation("title must not be empty".into()));
		}
		if trimmed.chars().count() > TITLE_MAX_LEN {
			return Err(DomainError::Validation(format!("title must be at most {TITLE_MAX_LEN} characters")));
		}
		Ok(Self(trimmed.to_owned()))
	}

	pub fn as_str(&self) -> &str {
		&self.0
	}

	pub fn into_string(self) -> String {
		self.0
	}
}

/// A validated URL slug: lowercase ASCII letters, digits and hyphens only, with
/// no leading or trailing hyphen.
#[derive(Debug, Clone)]
pub struct Slug(String);

impl Slug {
	pub fn parse(raw: String) -> Result<Self, DomainError> {
		let trimmed = raw.trim();
		if trimmed.is_empty() {
			return Err(DomainError::Validation("slug must not be empty".into()));
		}
		// Hand-rolled char scan — deliberately no regex dependency for this.
		if !trimmed.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
			return Err(DomainError::Validation("slug may contain only lowercase letters, digits and hyphens".into()));
		}
		if trimmed.starts_with('-') || trimmed.ends_with('-') {
			return Err(DomainError::Validation("slug must not start or end with a hyphen".into()));
		}
		Ok(Self(trimmed.to_owned()))
	}

	pub fn as_str(&self) -> &str {
		&self.0
	}

	pub fn into_string(self) -> String {
		self.0
	}
}

/// A blog post — a core domain entity. Only ever exists in a valid state: its
/// [`Title`] and [`Slug`] cannot be built without passing validation.
#[derive(Debug, Clone)]
pub struct Blog {
	pub id: Uuid,
	pub title: Title,
	pub slug: Slug,
	pub body: String,
	pub published: bool,
	pub created_at: DateTime<Utc>,
}

/// Data required to create a [`Blog`]. Stores already-parsed value objects, so
/// an invalid `NewBlog` is unrepresentable.
#[derive(Debug, Clone)]
pub struct NewBlog {
	pub title: Title,
	pub slug: Slug,
	pub body: String,
	pub published: bool,
}
