use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, postgres::PgPool};
use uuid::Uuid;

use crate::domain::{
	error::DomainError,
	model::blog::{Blog, NewBlog},
	port::blog_repository::BlogRepository,
};

/// Postgres-backed adapter implementing the [`BlogRepository`] port.
pub struct PostgresBlogRepository {
	pool: PgPool,
}

impl PostgresBlogRepository {
	pub fn new(pool: PgPool) -> Self {
		Self { pool }
	}
}

/// Row mirroring the `blogs` table. Private to the adapter so the domain model
/// stays free of persistence-specific derives.
#[derive(FromRow)]
struct BlogRow {
	id: Uuid,
	title: String,
	slug: String,
	body: String,
	published: bool,
	created_at: DateTime<Utc>,
}

impl From<BlogRow> for Blog {
	fn from(row: BlogRow) -> Self {
		Self { id: row.id, title: row.title, slug: row.slug, body: row.body, published: row.published, created_at: row.created_at }
	}
}

#[async_trait]
impl BlogRepository for PostgresBlogRepository {
	async fn create(&self, new_blog: NewBlog) -> Result<Blog, DomainError> {
		let row = sqlx::query_as::<_, BlogRow>("INSERT INTO blogs (title, slug, body, published) VALUES ($1, $2, $3, $4) RETURNING id, title, slug, body, published, created_at")
			.bind(&new_blog.title)
			.bind(&new_blog.slug)
			.bind(&new_blog.body)
			.bind(new_blog.published)
			.fetch_one(&self.pool)
			.await
			.map_err(map_sqlx_error)?;
		Ok(row.into())
	}

	async fn find_by_id(&self, id: Uuid) -> Result<Option<Blog>, DomainError> {
		let row = sqlx::query_as::<_, BlogRow>("SELECT id, title, slug, body, published, created_at FROM blogs WHERE id = $1")
			.bind(id)
			.fetch_optional(&self.pool)
			.await
			.map_err(map_sqlx_error)?;
		Ok(row.map(Into::into))
	}

	async fn list(&self) -> Result<Vec<Blog>, DomainError> {
		let rows = sqlx::query_as::<_, BlogRow>("SELECT id, title, slug, body, published, created_at FROM blogs ORDER BY created_at DESC")
			.fetch_all(&self.pool)
			.await
			.map_err(map_sqlx_error)?;
		Ok(rows.into_iter().map(Into::into).collect())
	}
}

/// Translate sqlx errors into domain errors, surfacing a unique violation as a
/// [`DomainError::Conflict`] rather than an opaque internal error.
fn map_sqlx_error(err: sqlx::Error) -> DomainError {
	if let sqlx::Error::Database(ref db_err) = err
		&& db_err.is_unique_violation()
	{
		return DomainError::Conflict("a blog with that slug already exists".into());
	}
	DomainError::Repository(anyhow!(err))
}
