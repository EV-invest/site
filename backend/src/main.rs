//! Composition root.
//!
//! Wiring lives here and only here: load config, build the driven adapters,
//! inject them into the use cases, mount the driving (HTTP) adapter, and serve.
//! The layered modules themselves live in the library crate (`lib.rs`).

use std::sync::Arc;

use anyhow::Context;
use backend::{
	api::{self, state::AppState},
	application::{blog_service::BlogService, ledger_service::LedgerService},
	config::AppConfig,
	infrastructure::{db, persistence::postgres_blog_repository::PostgresBlogRepository, tigerbeetle::tigerbeetle_ledger::TigerBeetleLedger},
};

// Sentry must be initialised before the async runtime starts — no #[tokio::main].
fn main() -> anyhow::Result<()> {
	dotenvy::dotenv().ok();

	let config = AppConfig::from_env().context("failed to load configuration")?;

	// Guard must stay alive for the duration of main — dropping it flushes events.
	let _sentry_guard = config.sentry_dsn.as_deref().map(|dsn| {
		sentry::init((
			dsn,
			sentry::ClientOptions {
				release: sentry::release_name!(),
				environment: Some(config.app_env.clone().into()),
				traces_sample_rate: if config.app_env == "production" { 0.1 } else { 1.0 },
				..Default::default()
			},
		))
	});

	init_tracing();

	tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.build()
		.context("failed to build tokio runtime")?
		.block_on(run(config))
}

async fn run(config: AppConfig) -> anyhow::Result<()> {
	let pool = db::connect(&config.database_url).await.context("failed to connect to the database")?;
	db::migrate(&pool).await.context("failed to run migrations")?;

	// TigerBeetle ledger.
	let ledger = TigerBeetleLedger::try_new(config.tigerbeetle_cluster_id, &config.tigerbeetle_address).context("failed to connect to TigerBeetle")?;
	let ledger_service = LedgerService::new(Arc::new(ledger));

	// Driven adapter ▶ use case ▶ shared state.
	let blog_repository = Arc::new(PostgresBlogRepository::new(pool));
	let blog_service = BlogService::new(blog_repository);
	let state = AppState::new(blog_service, ledger_service);

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
	tracing_subscriber::registry().with(filter).with(fmt::layer()).with(sentry::integrations::tracing::layer()).init();
}
