use axum::{
	Router,
	routing::{get, post},
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::{handler, openapi::ApiDoc, state::AppState};

/// Assemble the HTTP router: routes, shared state, and cross-cutting middleware.
/// API routes are nested under `/api/v1`; Swagger UI is served at `/swagger-ui`
/// and the raw spec at `/api-docs/openapi.json`.
pub fn build(state: AppState) -> Router {
	let routes = Router::new()
		.route("/health", get(handler::health::health))
		.route("/blogs", post(handler::blog::create_blog).get(handler::blog::list_blogs))
		.route("/blogs/{id}", get(handler::blog::get_blog));

	Router::new()
		.nest("/api/v1", routes)
		.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
		.layer(TraceLayer::new_for_http())
		.layer(CorsLayer::permissive())
		.with_state(state)
}
