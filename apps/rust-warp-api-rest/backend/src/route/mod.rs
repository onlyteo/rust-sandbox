mod api_endpoints;
mod static_resources;

use crate::route::api_endpoints::api_endpoints_filter;
use crate::route::static_resources::static_resources_filter;
use warp::Filter;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    api_endpoints_filter().or(static_resources_filter())
}
