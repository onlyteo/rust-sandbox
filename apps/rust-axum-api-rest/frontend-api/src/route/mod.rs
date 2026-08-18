mod api;
mod resources;

use crate::route::api::api_routes;
use crate::route::resources::resource_routes;
use axum::Router;

pub fn routes() -> Router {
    Router::new().merge(api_routes()).merge(resource_routes())
}
