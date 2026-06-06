use async_trait::async_trait;
use uuid::Uuid;

use domain::{
	error::DomainError,
	model::blog::{Blog, NewBlog},
};

/// Outbound port for persisting and retrieving [`Blog`]s.
///
/// The core depends on this abstraction; concrete adapters (e.g. Postgres) live
/// in `crate::infrastructure` and implement it. `#[async_trait]` keeps the trait
/// object-safe so it can be held behind `Arc<dyn BlogRepository>`.
#[async_trait]
pub trait BlogRepository: Send + Sync {
	async fn create(&self, new_blog: NewBlog) -> Result<Blog, DomainError>;

	async fn find_by_id(&self, id: Uuid) -> Result<Option<Blog>, DomainError>;

	/// Page through blogs. `limit`/`offset` are passed straight to the store;
	/// callers are expected to supply sane, bounded values.
	async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Blog>, DomainError>;
}
