use axum::Router;
use tower_http::services::ServeDir;

pub fn resource_routes() -> Router {
    let serve_dir = ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/static"));
    Router::new().fallback_service(serve_dir)
}
