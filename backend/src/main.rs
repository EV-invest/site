//! Composition root.
//!
//! Wiring lives here and only here: load config, build the driven adapters,
//! inject them into the use cases, mount the driving (HTTP) adapter, and serve.
//!
//! Dependency direction (hexagonal):
//!   api ─▶ application ─▶ domain ◀─ infrastructure
//! Everything points inward at `domain`; nothing in `domain` points out.

mod api;
mod application;
mod config;
mod domain;
mod infrastructure;

use std::sync::Arc;

use anyhow::Context;

use crate::{
	api::state::AppState,
	application::blog_service::BlogService,
	config::AppConfig,
	infrastructure::{db, persistence::postgres_blog_repository::PostgresBlogRepository},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	dotenvy::dotenv().ok();
	init_tracing();

	let config = AppConfig::from_env().context("failed to load configuration")?;

	let pool = db::connect(&config.database_url).await.context("failed to connect to the database")?;
	db::migrate(&pool).await.context("failed to run migrations")?;

	// Driven adapter ▶ use case ▶ shared state.
	let blog_repository = Arc::new(PostgresBlogRepository::new(pool));
	let blog_service = BlogService::new(blog_repository);
	let state = AppState::new(blog_service);

	let router = api::router::build(state);

	let listener = tokio::net::TcpListener::bind(config.bind_addr)
		.await
		.with_context(|| format!("failed to bind {}", config.bind_addr))?;
	tracing::info!(addr = %config.bind_addr, "backend listening");
	axum::serve(listener, router).await.context("server error")?;

	Ok(())
}

fn init_tracing() {
	use tracing_subscriber::{EnvFilter, fmt, prelude::*};

	let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,backend=debug"));
	tracing_subscriber::registry().with(filter).with(fmt::layer()).init();
}
