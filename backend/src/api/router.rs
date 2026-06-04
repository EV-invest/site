use axum::{
	Router,
	routing::{get, post},
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::api::{handler, state::AppState};

/// Assemble the HTTP router: routes, shared state, and cross-cutting middleware.
/// Everything is nested under `/api/v1`.
pub fn build(state: AppState) -> Router {
	let routes = Router::new()
		.route("/health", get(handler::health::health))
		.route("/blogs", post(handler::blog::create_blog).get(handler::blog::list_blogs))
		.route("/blogs/{id}", get(handler::blog::get_blog));

	Router::new()
		.nest("/api/v1", routes)
		.layer(TraceLayer::new_for_http())
		.layer(CorsLayer::permissive())
		.with_state(state)
}
