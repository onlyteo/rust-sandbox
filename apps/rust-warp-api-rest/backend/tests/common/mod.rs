#![allow(dead_code)]
use rust_warp_rest_api_backend::repository::order::OrderRepository;
use rust_warp_rest_api_backend::repository::product::ProductRepository;
use rust_warp_rest_api_backend::route;
use rust_warp_rest_api_backend::service::order::OrderService;
use rust_warp_rest_api_backend::service::product::ProductService;
use rust_warp_rest_api_backend::state::AppState;
use serde::de::DeserializeOwned;
use warp::http::Response;
use warp::hyper::body::Bytes;
use warp::{Filter, Reply};

pub const SEEDED_PRODUCT_ID: &str = "6821a470-071b-4cf4-b993-43cc8b2a8827";
pub const SKYWALKER: &str = "Basic c2t5d2Fsa2VyOnNlY3JldA==";
pub const VADER: &str = "Basic dmFkZXI6c2VjcmV0";

pub fn app() -> impl Filter<Extract = (impl Reply,), Error = std::convert::Infallible> + Clone {
    let order_service = OrderService::new(OrderRepository::new());
    let product_service = ProductService::new(ProductRepository::new());
    route::routes(AppState::new(order_service, product_service))
}

pub async fn send<F>(
    app: &F,
    method: &str,
    uri: &str,
    auth: Option<&str>,
    body: Option<serde_json::Value>,
) -> Response<Bytes>
where
    F: Filter<Error = std::convert::Infallible> + 'static,
    F::Extract: Reply + Send,
{
    let mut request = warp::test::request().method(method).path(uri);
    if let Some(auth) = auth {
        request = request.header("authorization", auth);
    }
    if let Some(body) = body {
        request = request.json(&body);
    }
    request.reply(app).await
}

pub fn parse<T: DeserializeOwned>(response: &Response<Bytes>) -> T {
    serde_json::from_slice(response.body()).unwrap()
}
