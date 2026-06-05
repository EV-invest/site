use utoipa::OpenApi;

use crate::api::{
	dto::blog::{BlogResponse, CreateBlogRequest},
	handler,
};

/// Root of the generated OpenAPI 3.1 document — the single source of truth for
/// the HTTP contract. Paths are pulled from the `#[utoipa::path]` macros on the
/// handlers; schemas from the `ToSchema` derives on the DTOs.
#[derive(OpenApi)]
#[openapi(
	info(title = "EV blogs API", description = "Source of truth for blog posts", version = env!("CARGO_PKG_VERSION")),
	paths(handler::health::health, handler::blog::create_blog, handler::blog::get_blog, handler::blog::list_blogs),
	components(schemas(CreateBlogRequest, BlogResponse)),
	tags((name = "blogs", description = "Blog posts"), (name = "health", description = "Liveness")),
)]
pub struct ApiDoc;
