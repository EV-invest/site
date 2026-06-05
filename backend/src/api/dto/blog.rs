use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::domain::{
	error::DomainError,
	model::blog::{Blog, NewBlog, Slug, Title},
};

/// Inbound payload for `POST /blogs`. Carries raw strings off the wire; parsing
/// into validated value objects happens in the `TryFrom` below — at the boundary.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateBlogRequest {
	#[schema(example = "Hello, world")]
	pub title: String,
	#[schema(example = "hello-world")]
	pub slug: String,
	#[schema(example = "My first post.")]
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
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListBlogsQuery {
	#[serde(default = "default_limit")]
	#[param(default = 20, minimum = 1, maximum = 100)]
	pub limit: i64,
	#[serde(default)]
	#[param(default = 0, minimum = 0)]
	pub offset: i64,
}

fn default_limit() -> i64 {
	20
}

/// Outbound representation of a [`Blog`] on the wire.
#[derive(Debug, Serialize, ToSchema)]
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
