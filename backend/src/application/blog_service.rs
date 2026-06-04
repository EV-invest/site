use std::sync::Arc;

use uuid::Uuid;

use crate::domain::{
	error::DomainError,
	model::blog::{Blog, NewBlog},
	port::blog_repository::BlogRepository,
};

/// Use cases operating on [`Blog`]s. Depends only on the [`BlogRepository`]
/// port, so it is agnostic to the concrete persistence adapter and trivially
/// testable with an in-memory fake.
#[derive(Clone)]
pub struct BlogService {
	repository: Arc<dyn BlogRepository>,
}

impl BlogService {
	pub fn new(repository: Arc<dyn BlogRepository>) -> Self {
		Self { repository }
	}

	pub async fn create(&self, new_blog: NewBlog) -> Result<Blog, DomainError> {
		if new_blog.title.trim().is_empty() {
			return Err(DomainError::Validation("title must not be empty".into()));
		}
		if new_blog.slug.trim().is_empty() {
			return Err(DomainError::Validation("slug must not be empty".into()));
		}
		self.repository.create(new_blog).await
	}

	pub async fn get(&self, id: Uuid) -> Result<Blog, DomainError> {
		self.repository.find_by_id(id).await?.ok_or(DomainError::NotFound)
	}

	pub async fn list(&self) -> Result<Vec<Blog>, DomainError> {
		self.repository.list().await
	}
}
