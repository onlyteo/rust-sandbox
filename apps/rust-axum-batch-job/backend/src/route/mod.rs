use axum::Router;
use axum::routing::get;

pub fn routes() -> Router {
    Router::new().route("/health", get(|| async { "OK" }))
}
