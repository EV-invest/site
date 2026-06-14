use async_trait::async_trait;
use domain::{
	architecture::{Reader, Repository},
	error::DomainError,
	model::blog::{Blog, BlogId, NewBlog},
};

/// Outbound port for persisting and retrieving [`Blog`]s.
///
/// A named-alias super-trait over the `architecture` markers: it ties the port to the
/// `Blog` aggregate (living documentation) while the concrete CRUD methods live
/// here, where the typed [`BlogId`] is known. The core depends on this
/// abstraction; concrete adapters (e.g. Postgres) live in `crate::infrastructure`
/// and implement it. `#[async_trait]` keeps the trait object-safe so it can be
/// held behind `Arc<dyn BlogRepository>`.
///
/// The [`Reader`] super-trait is a reserved CQRS/ISP read-split seam: today
/// every consumer takes the full `Arc<dyn BlogRepository>`, so nothing yet
/// depends on the narrow read port. It is kept so the first query handler that
/// needs read-only power can take `Arc<dyn Reader<Aggregate = Blog>>` without
/// reworking the port hierarchy — not yet exercised by a dedicated read model.
#[async_trait]
pub trait BlogRepository: Repository<Aggregate = Blog> + Reader<Aggregate = Blog> {
	async fn create(&self, new_blog: NewBlog) -> Result<Blog, DomainError>;

	async fn find_by_id(&self, id: BlogId) -> Result<Option<Blog>, DomainError>;

	/// Page through blogs. `limit`/`offset` are passed straight to the store;
	/// callers are expected to supply sane, bounded values.
	async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Blog>, DomainError>;
}
