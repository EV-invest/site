use sqlx::postgres::{PgPool, PgPoolOptions};

/// Open a connection pool to Postgres.
pub async fn connect(database_url: &str) -> anyhow::Result<PgPool> {
	let pool = PgPoolOptions::new().max_connections(10).connect(database_url).await?;
	Ok(pool)
}

/// Apply any pending migrations embedded from the `migrations/` directory.
pub async fn migrate(pool: &PgPool) -> anyhow::Result<()> {
	sqlx::migrate!("./migrations").run(pool).await?;
	Ok(())
}
