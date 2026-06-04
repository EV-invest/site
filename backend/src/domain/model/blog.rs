use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A blog post — a core domain entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blog {
	pub id: Uuid,
	pub title: String,
	pub slug: String,
	pub body: String,
	pub published: bool,
	pub created_at: DateTime<Utc>,
}

/// Data required to create a [`Blog`]. Identity and timestamps are assigned by
/// the persistence adapter, not the caller.
#[derive(Debug, Clone)]
pub struct NewBlog {
	pub title: String,
	pub slug: String,
	pub body: String,
	pub published: bool,
}
