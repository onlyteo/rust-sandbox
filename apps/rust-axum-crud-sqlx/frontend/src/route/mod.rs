mod resources;

use crate::route::resources::resource_routes;
use axum::Router;

pub fn routes() -> Router {
    Router::new().merge(resource_routes())
}
