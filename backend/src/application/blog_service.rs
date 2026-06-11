use std::sync::Arc;

use domain::{
	error::DomainError,
	model::blog::{Blog, NewBlog},
};
use uuid::Uuid;

use crate::domain::port::blog_repository::BlogRepository;

/// Use cases operating on [`Blog`]s. Depends only on the [`BlogRepository`]
/// port, so it is agnostic to the concrete adapter and trivially testable.
#[derive(Clone)]
pub struct BlogService {
	repository: Arc<dyn BlogRepository>,
}

impl BlogService {
	pub fn new(repository: Arc<dyn BlogRepository>) -> Self {
		Self { repository }
	}

	/// Persist a new blog. Validation already happened when the caller built the
	/// [`NewBlog`]'s value objects, so this is a thin passthrough — correct for
	/// plain CRUD, with no artificial rules bolted on.
	pub async fn create(&self, new_blog: NewBlog) -> Result<Blog, DomainError> {
		self.repository.create(new_blog).await
	}

	pub async fn get(&self, id: Uuid) -> Result<Blog, DomainError> {
		self.repository.find_by_id(id).await?.ok_or(DomainError::NotFound { entity: "blog", id })
	}

	pub async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Blog>, DomainError> {
		self.repository.list(limit, offset).await
	}
}

#[cfg(test)]
mod tests {
	use std::sync::{Arc, Mutex};

	use async_trait::async_trait;
	use domain::model::blog::{Slug, Title};
	use jiff::Timestamp;

	use super::*;

	/// In-memory fake of the repository port, to exercise the service without
	/// touching Postgres.
	#[derive(Default)]
	struct InMemoryBlogRepository {
		blogs: Mutex<Vec<Blog>>,
	}

	#[async_trait]
	impl BlogRepository for InMemoryBlogRepository {
		async fn create(&self, new_blog: NewBlog) -> Result<Blog, DomainError> {
			let blog = Blog {
				id: Uuid::new_v4(),
				title: new_blog.title,
				slug: new_blog.slug,
				body: new_blog.body,
				published: new_blog.published,
				created_at: Timestamp::now(),
			};
			self.blogs.lock().unwrap().push(blog.clone());
			Ok(blog)
		}

		async fn find_by_id(&self, id: Uuid) -> Result<Option<Blog>, DomainError> {
			Ok(self.blogs.lock().unwrap().iter().find(|b| b.id == id).cloned())
		}

		async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Blog>, DomainError> {
			let blogs = self.blogs.lock().unwrap();
			Ok(blogs.iter().skip(offset.max(0) as usize).take(limit.max(0) as usize).cloned().collect())
		}
	}

	fn service() -> BlogService {
		BlogService::new(Arc::new(InMemoryBlogRepository::default()))
	}

	fn sample_new_blog() -> NewBlog {
		NewBlog {
			title: Title::parse("Hello World".into()).unwrap(),
			slug: Slug::parse("hello-world".into()).unwrap(),
			body: "first post".into(),
			published: true,
		}
	}

	#[test]
	fn invalid_title_is_rejected() {
		assert!(matches!(Title::parse("   ".into()), Err(DomainError::Validation(_))));
		assert!(matches!(Title::parse("x".repeat(201)), Err(DomainError::Validation(_))));
	}

	#[test]
	fn invalid_slug_is_rejected() {
		assert!(matches!(Slug::parse("Hello World".into()), Err(DomainError::Validation(_))));
		assert!(matches!(Slug::parse("UPPER".into()), Err(DomainError::Validation(_))));
		assert!(matches!(Slug::parse("-leading".into()), Err(DomainError::Validation(_))));
		assert!(matches!(Slug::parse("trailing-".into()), Err(DomainError::Validation(_))));
	}

	#[tokio::test]
	async fn get_missing_blog_returns_not_found() {
		let svc = service();
		let err = svc.get(Uuid::new_v4()).await.unwrap_err();
		assert!(matches!(err, DomainError::NotFound { entity: "blog", .. }));
	}

	#[tokio::test]
	async fn create_then_get_roundtrips() {
		let svc = service();
		let created = svc.create(sample_new_blog()).await.unwrap();
		let fetched = svc.get(created.id).await.unwrap();
		assert_eq!(created.id, fetched.id);
		assert_eq!(fetched.slug.as_str(), "hello-world");
	}

	#[tokio::test]
	async fn list_respects_limit_and_offset() {
		let svc = service();
		for i in 0..3 {
			let nb = NewBlog {
				title: Title::parse(format!("Post {i}")).unwrap(),
				slug: Slug::parse(format!("post-{i}")).unwrap(),
				body: String::new(),
				published: false,
			};
			svc.create(nb).await.unwrap();
		}
		assert_eq!(svc.list(2, 0).await.unwrap().len(), 2);
		assert_eq!(svc.list(2, 2).await.unwrap().len(), 1);
	}
}
