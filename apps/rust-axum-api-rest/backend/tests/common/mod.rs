#![allow(dead_code)]
use axum::Router;
use axum::body::Body;
use axum::http::{Method, Request, Response};
use http_body_util::BodyExt;
use rust_axum_rest_api_backend::repository::order::OrderRepository;
use rust_axum_rest_api_backend::repository::product::ProductRepository;
use rust_axum_rest_api_backend::route;
use rust_axum_rest_api_backend::service::order::OrderService;
use rust_axum_rest_api_backend::service::product::ProductService;
use rust_axum_rest_api_backend::state::AppState;
use serde::de::DeserializeOwned;
use tower::ServiceExt;

pub const SEEDED_PRODUCT_ID: &str = "6821a470-071b-4cf4-b993-43cc8b2a8827";
pub const SKYWALKER: &str = "Basic c2t5d2Fsa2VyOnNlY3JldA==";
pub const VADER: &str = "Basic dmFkZXI6c2VjcmV0";

pub fn app() -> Router {
    let order_service = OrderService::new(OrderRepository::new());
    let product_service = ProductService::new(ProductRepository::new());
    route::routes(AppState::new(order_service, product_service))
}

pub async fn send(
    app: &Router,
    method: Method,
    uri: &str,
    auth: &str,
    body: Option<serde_json::Value>,
) -> Response<Body> {
    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header("authorization", auth);
    let body = match body {
        Some(json) => {
            builder = builder.header("content-type", "application/json");
            Body::from(json.to_string())
        }
        None => Body::empty(),
    };
    app.clone()
        .oneshot(builder.body(body).unwrap())
        .await
        .unwrap()
}

pub async fn parse<T: DeserializeOwned>(response: Response<Body>) -> T {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}
