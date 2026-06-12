use crate::application::{blog_service::BlogService, ledger_service::LedgerService};

/// Shared application state injected into handlers via axum's `State` extractor.
/// Holds the use-case services; cloning is cheap (services wrap `Arc`s).
#[derive(Clone)]
pub struct AppState {
	pub blog_service: BlogService,
	pub ledger_service: LedgerService,
}

impl AppState {
	pub fn new(blog_service: BlogService, ledger_service: LedgerService) -> Self {
		Self { blog_service, ledger_service }
	}
}
