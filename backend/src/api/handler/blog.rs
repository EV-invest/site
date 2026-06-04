use axum::{
	Json,
	extract::{Path, State},
	http::StatusCode,
};
use uuid::Uuid;

use crate::api::{
	dto::blog::{BlogResponse, CreateBlogRequest},
	error::ApiError,
	state::AppState,
};

/// `POST /blogs` — create a blog post.
pub async fn create_blog(State(state): State<AppState>, Json(payload): Json<CreateBlogRequest>) -> Result<(StatusCode, Json<BlogResponse>), ApiError> {
	let blog = state.blog_service.create(payload.into()).await?;
	Ok((StatusCode::CREATED, Json(blog.into())))
}

/// `GET /blogs/{id}` — fetch a single blog post.
pub async fn get_blog(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<BlogResponse>, ApiError> {
	let blog = state.blog_service.get(id).await?;
	Ok(Json(blog.into()))
}

/// `GET /blogs` — list blog posts.
pub async fn list_blogs(State(state): State<AppState>) -> Result<Json<Vec<BlogResponse>>, ApiError> {
	let blogs = state.blog_service.list().await?;
	Ok(Json(blogs.into_iter().map(Into::into).collect()))
}
