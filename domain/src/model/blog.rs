use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::DomainError;

const TITLE_MAX_LEN: usize = 200;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Title(String);

impl Title {
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Slug(String);

impl Slug {
	pub fn parse(raw: String) -> Result<Self, DomainError> {
		let trimmed = raw.trim();
		if trimmed.is_empty() {
			return Err(DomainError::Validation("slug must not be empty".into()));
		}
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Blog {
	pub id: Uuid,
	pub title: Title,
	pub slug: Slug,
	pub body: String,
	pub published: bool,
	pub created_at: Timestamp,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewBlog {
	pub title: Title,
	pub slug: Slug,
	pub body: String,
	pub published: bool,
}
