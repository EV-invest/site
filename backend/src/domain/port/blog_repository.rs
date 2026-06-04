use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
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

	async fn list(&self) -> Result<Vec<Blog>, DomainError>;
}
