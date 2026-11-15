mod common;

use common::*;
use rust_warp_rest_api_backend::model::Product;
use warp::http::StatusCode;

#[tokio::test]
async fn list_products_returns_seeded_products_ordered_by_name() {
    let app = app();
    let response = send(&app, "GET", "/api/products", Some(SKYWALKER), None).await;
    assert_eq!(response.status(), StatusCode::OK);

    let products: Vec<Product> = parse(&response);
    assert_eq!(products.len(), 6);
    assert!(products.windows(2).all(|w| w[0].name <= w[1].name));
}

#[tokio::test]
async fn list_products_requires_authentication() {
    let app = app();
    let response = send(&app, "GET", "/api/products", None, None).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        response.headers()["content-type"],
        "application/problem+json"
    );
    let problem: serde_json::Value = parse(&response);
    assert_eq!(problem["instance"], "/api/products");
}

#[tokio::test]
async fn index_page_is_public() {
    let app = app();
    let response = send(&app, "GET", "/", None, None).await;
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn unknown_api_path_returns_problem() {
    let app = app();
    let response = send(&app, "GET", "/api/unknown", Some(SKYWALKER), None).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        response.headers()["content-type"],
        "application/problem+json"
    );
}
