use async_trait::async_trait;
use domain::{
	error::DomainError,
	model::blog::{Blog, NewBlog, Slug, Title},
};
use jiff::Timestamp;
use sqlx::{FromRow, postgres::PgPool};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::port::blog_repository::BlogRepository;

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
	created_at: OffsetDateTime,
}

impl TryFrom<BlogRow> for Blog {
	type Error = DomainError;

	/// Rebuild the domain entity from a row. Stored values were validated on the
	/// way in, so a parse failure here means the row is corrupt — reported as a
	/// repository error, never as a client-facing validation error.
	fn try_from(row: BlogRow) -> Result<Self, Self::Error> {
		Ok(Self {
			id: row.id,
			title: Title::parse(row.title).map_err(corrupt_row)?,
			slug: Slug::parse(row.slug).map_err(corrupt_row)?,
			body: row.body,
			published: row.published,
			created_at: Timestamp::new(row.created_at.unix_timestamp(), row.created_at.nanosecond() as i32).map_err(|e| DomainError::Repository(format!("invalid timestamp: {e}")))?,
		})
	}
}

#[async_trait]
impl BlogRepository for PostgresBlogRepository {
	async fn create(&self, new_blog: NewBlog) -> Result<Blog, DomainError> {
		let row = sqlx::query_as::<_, BlogRow>("INSERT INTO blogs (title, slug, body, published) VALUES ($1, $2, $3, $4) RETURNING id, title, slug, body, published, created_at")
			.bind(new_blog.title.as_str())
			.bind(new_blog.slug.as_str())
			.bind(&new_blog.body)
			.bind(new_blog.published)
			.fetch_one(&self.pool)
			.await
			.map_err(map_sqlx_error)?;
		row.try_into()
	}

	async fn find_by_id(&self, id: Uuid) -> Result<Option<Blog>, DomainError> {
		let row = sqlx::query_as::<_, BlogRow>("SELECT id, title, slug, body, published, created_at FROM blogs WHERE id = $1")
			.bind(id)
			.fetch_optional(&self.pool)
			.await
			.map_err(map_sqlx_error)?;
		row.map(Blog::try_from).transpose()
	}

	async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Blog>, DomainError> {
		let rows = sqlx::query_as::<_, BlogRow>("SELECT id, title, slug, body, published, created_at FROM blogs ORDER BY created_at DESC LIMIT $1 OFFSET $2")
			.bind(limit)
			.bind(offset)
			.fetch_all(&self.pool)
			.await
			.map_err(map_sqlx_error)?;
		rows.into_iter().map(Blog::try_from).collect()
	}
}

/// A persisted row failed domain re-validation: the database holds bad data.
fn corrupt_row(err: DomainError) -> DomainError {
	DomainError::Repository(format!("corrupt blog row: {err}"))
}

/// Translate sqlx errors into domain errors. A unique violation is an honest
/// `Conflict` (→ 409); everything else is an opaque `Repository` (→ 500). A
/// database failure is never reported as a `Validation`.
fn map_sqlx_error(err: sqlx::Error) -> DomainError {
	if let sqlx::Error::Database(ref db_err) = err
		&& db_err.is_unique_violation()
	{
		return DomainError::Conflict("a blog with that slug already exists".into());
	}
	DomainError::Repository(err.to_string())
}
