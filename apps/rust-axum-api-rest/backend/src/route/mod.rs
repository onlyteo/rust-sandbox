mod api;
mod resources;

use crate::route::api::post_greeting;
use crate::route::resources::get_resources;
use axum::routing::{get, post};
use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .route("/api/greetings", post(post_greeting))
        .route("/", get(get_resources))
}
