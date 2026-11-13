mod common;

use axum::http::{Method, StatusCode};
use common::*;
use rust_axum_rest_api_backend::model::Product;

#[tokio::test]
async fn list_products_returns_seeded_products_ordered_by_name() {
    let app = app();
    let response = send(&app, Method::GET, "/api/products", SKYWALKER, None).await;
    assert_eq!(response.status(), StatusCode::OK);

    let products: Vec<Product> = parse(response).await;
    assert_eq!(products.len(), 6);
    assert!(products.windows(2).all(|w| w[0].name <= w[1].name));
}

#[tokio::test]
async fn list_products_requires_authentication() {
    let app = app();
    let response = send(&app, Method::GET, "/api/products", "", None).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
