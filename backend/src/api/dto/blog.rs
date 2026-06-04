use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{
	error::DomainError,
	model::blog::{Blog, NewBlog, Slug, Title},
};

/// Inbound payload for `POST /blogs`. Carries raw strings off the wire; parsing
/// into validated value objects happens in the `TryFrom` below — at the boundary.
#[derive(Debug, Deserialize)]
pub struct CreateBlogRequest {
	pub title: String,
	pub slug: String,
	pub body: String,
	#[serde(default)]
	pub published: bool,
}

impl TryFrom<CreateBlogRequest> for NewBlog {
	type Error = DomainError;

	fn try_from(req: CreateBlogRequest) -> Result<Self, Self::Error> {
		Ok(Self {
			title: Title::parse(req.title)?,
			slug: Slug::parse(req.slug)?,
			body: req.body,
			published: req.published,
		})
	}
}

/// Query parameters for `GET /blogs`, with sane defaults (limit 20, offset 0).
#[derive(Debug, Deserialize)]
pub struct ListBlogsQuery {
	#[serde(default = "default_limit")]
	pub limit: i64,
	#[serde(default)]
	pub offset: i64,
}

fn default_limit() -> i64 {
	20
}

/// Outbound representation of a [`Blog`] on the wire.
#[derive(Debug, Serialize)]
pub struct BlogResponse {
	pub id: Uuid,
	pub title: String,
	pub slug: String,
	pub body: String,
	pub published: bool,
	pub created_at: DateTime<Utc>,
}

impl From<Blog> for BlogResponse {
	fn from(blog: Blog) -> Self {
		Self {
			id: blog.id,
			title: blog.title.into_string(),
			slug: blog.slug.into_string(),
			body: blog.body,
			published: blog.published,
			created_at: blog.created_at,
		}
	}
}
