use std::{env, net::SocketAddr};

use anyhow::Context;

/// Application configuration, sourced from environment variables (and `.env`
/// in development via `dotenvy`).
#[derive(Debug, Clone)]
pub struct AppConfig {
	pub database_url: String,
	pub bind_addr: SocketAddr,
}

impl AppConfig {
	pub fn from_env() -> anyhow::Result<Self> {
		let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
		let bind_addr = env::var("BIND_ADDR")
			.unwrap_or_else(|_| "0.0.0.0:8080".to_string())
			.parse()
			.context("BIND_ADDR must be a valid socket address, e.g. 0.0.0.0:8080")?;
		Ok(Self { database_url, bind_addr })
	}
}
