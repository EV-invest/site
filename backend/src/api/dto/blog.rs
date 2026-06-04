use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::model::blog::{Blog, NewBlog};

/// Inbound payload for `POST /blogs`.
#[derive(Debug, Deserialize)]
pub struct CreateBlogRequest {
	pub title: String,
	pub slug: String,
	pub body: String,
	#[serde(default)]
	pub published: bool,
}

impl From<CreateBlogRequest> for NewBlog {
	fn from(req: CreateBlogRequest) -> Self {
		Self { title: req.title, slug: req.slug, body: req.body, published: req.published }
	}
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
		Self { id: blog.id, title: blog.title, slug: blog.slug, body: blog.body, published: blog.published, created_at: blog.created_at }
	}
}
