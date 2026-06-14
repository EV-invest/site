use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
	architecture::{AggregateRoot, Entity, Id},
	error::DomainError,
};

const TITLE_MAX_LEN: usize = 200;
const BODY_MAX_LEN: usize = 100_000;

/// Unique identifier for a [`Blog`].
pub type BlogId = Id<BlogTag, Uuid>;
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
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

/// The blog body. A value object so the one real invariant — a size cap — is
/// enforced at the boundary instead of letting an unbounded `String` through.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Body(String);

impl Body {
	pub fn parse(raw: String) -> Result<Self, DomainError> {
		if raw.len() > BODY_MAX_LEN {
			return Err(DomainError::Validation(format!("body must be at most {BODY_MAX_LEN} bytes")));
		}
		Ok(Self(raw))
	}

	pub fn as_str(&self) -> &str {
		&self.0
	}

	pub fn into_string(self) -> String {
		self.0
	}
}

/// Tag giving the blog its own identity newtype.
pub struct BlogTag;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Blog {
	pub id: BlogId,
	pub title: Title,
	pub slug: Slug,
	pub body: Body,
	pub published: bool,
	pub created_at: Timestamp,
}

impl Entity for Blog {
	type Id = BlogId;

	fn id(&self) -> BlogId {
		self.id
	}
}

impl AggregateRoot for Blog {
	const NAME: &'static str = "blog";
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewBlog {
	pub title: Title,
	pub slug: Slug,
	pub body: Body,
	pub published: bool,
}
