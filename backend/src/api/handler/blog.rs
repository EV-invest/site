use axum::{
	Json,
	extract::{Path, Query, State},
	http::StatusCode,
};
use uuid::Uuid;

use crate::{
	api::{
		dto::blog::{BlogResponse, CreateBlogRequest, ListBlogsQuery},
		error::ApiError,
		state::AppState,
	},
	domain::model::blog::NewBlog,
};

/// `POST /blogs` — create a blog post. The DTO is parsed into validated value
/// objects here; a validation error surfaces at this boundary as 400.
pub async fn create_blog(State(state): State<AppState>, Json(payload): Json<CreateBlogRequest>) -> Result<(StatusCode, Json<BlogResponse>), ApiError> {
	let new_blog = NewBlog::try_from(payload)?;
	let blog = state.blog_service.create(new_blog).await?;
	Ok((StatusCode::CREATED, Json(blog.into())))
}

/// `GET /blogs/{id}` — fetch a single blog post.
pub async fn get_blog(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<BlogResponse>, ApiError> {
	let blog = state.blog_service.get(id).await?;
	Ok(Json(blog.into()))
}

/// `GET /blogs?limit=&offset=` — list blog posts. Bounds are clamped so a
/// mistaken or hostile query can't issue a negative or oversized LIMIT/OFFSET.
pub async fn list_blogs(State(state): State<AppState>, Query(query): Query<ListBlogsQuery>) -> Result<Json<Vec<BlogResponse>>, ApiError> {
	let limit = query.limit.clamp(1, 100);
	let offset = query.offset.max(0);
	let blogs = state.blog_service.list(limit, offset).await?;
	Ok(Json(blogs.into_iter().map(Into::into).collect()))
}
