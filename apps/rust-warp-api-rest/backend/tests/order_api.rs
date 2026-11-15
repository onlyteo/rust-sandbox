mod common;

use common::*;
use rust_warp_rest_api_backend::model::{Order, OrderProduct, OrderStatus};
use serde_json::json;
use uuid::Uuid;
use warp::http::StatusCode;

fn seeded_id() -> Uuid {
    Uuid::parse_str(SEEDED_PRODUCT_ID).unwrap()
}

async fn create<F>(app: &F, auth: &str, quantity: i32) -> Order
where
    F: warp::Filter<Error = std::convert::Infallible> + 'static,
    F::Extract: warp::Reply + Send,
{
    let body = json!({ "products": [{ "id": SEEDED_PRODUCT_ID, "quantity": quantity }] });
    let response = send(app, "POST", "/api/orders", Some(auth), Some(body)).await;
    assert_eq!(response.status(), StatusCode::CREATED);
    parse(&response)
}

#[tokio::test]
async fn create_order_returns_open_order() {
    let app = app();
    let order = create(&app, SKYWALKER, 2).await;
    assert_eq!(order.status, OrderStatus::Open);
    assert_eq!(order.username, "skywalker");
    assert_eq!(order.products, vec![OrderProduct::new(seeded_id(), 2)]);
}

#[tokio::test]
async fn second_open_order_conflicts() {
    let app = app();
    create(&app, SKYWALKER, 1).await;
    let body = json!({ "products": [] });
    let response = send(&app, "POST", "/api/orders", Some(SKYWALKER), Some(body)).await;
    assert_eq!(response.status(), StatusCode::CONFLICT);
    let problem: serde_json::Value = parse(&response);
    assert_eq!(problem["type"], "urn:app:order-already-exists");
    assert_eq!(problem["instance"], "/api/orders");
}

#[tokio::test]
async fn new_open_order_allowed_after_completion() {
    let app = app();
    let order = create(&app, SKYWALKER, 1).await;
    let body =
        json!({ "status": "COMPLETED", "products": [{ "id": SEEDED_PRODUCT_ID, "quantity": 1 }] });
    let uri = format!("/api/orders/{}", order.id);
    send(&app, "PUT", &uri, Some(SKYWALKER), Some(body)).await;
    create(&app, SKYWALKER, 1).await;
}

#[tokio::test]
async fn get_order_by_id_matches_created_order() {
    let app = app();
    let created = create(&app, SKYWALKER, 1).await;
    let uri = format!("/api/orders/{}", created.id);
    let response = send(&app, "GET", &uri, Some(SKYWALKER), None).await;
    assert_eq!(response.status(), StatusCode::OK);
    let fetched: Order = parse(&response);
    assert_eq!(fetched.id, created.id);
}

#[tokio::test]
async fn get_missing_order_returns_not_found() {
    let app = app();
    let uri = format!("/api/orders/{}", Uuid::new_v4());
    let response = send(&app, "GET", &uri, Some(SKYWALKER), None).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let problem: serde_json::Value = parse(&response);
    assert_eq!(problem["type"], "urn:app:order-not-found");
    assert_eq!(problem["instance"], uri);
}

#[tokio::test]
async fn orders_are_scoped_to_user() {
    let app = app();
    let created = create(&app, SKYWALKER, 1).await;

    let uri = format!("/api/orders/{}", created.id);
    let response = send(&app, "GET", &uri, Some(VADER), None).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = json!({ "status": "CANCELLED", "products": [] });
    let response = send(&app, "PUT", &uri, Some(VADER), Some(body)).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let response = send(&app, "GET", "/api/orders", Some(VADER), None).await;
    let orders: Vec<Order> = parse(&response);
    assert!(orders.is_empty());
}

#[tokio::test]
async fn update_syncs_products_and_status() {
    let app = app();
    let other = "d2ac5e0c-83b0-4934-a975-a2134fc1e767";
    let body = json!({ "products": [
        { "id": SEEDED_PRODUCT_ID, "quantity": 1 },
        { "id": other, "quantity": 5 }
    ]});
    let response = send(&app, "POST", "/api/orders", Some(SKYWALKER), Some(body)).await;
    let created: Order = parse(&response);
    assert_eq!(created.products.len(), 2);

    let body =
        json!({ "status": "COMPLETED", "products": [{ "id": SEEDED_PRODUCT_ID, "quantity": 3 }] });
    let uri = format!("/api/orders/{}", created.id);
    let response = send(&app, "PUT", &uri, Some(SKYWALKER), Some(body)).await;
    assert_eq!(response.status(), StatusCode::OK);
    let updated: Order = parse(&response);
    assert_eq!(updated.status, OrderStatus::Completed);
    assert_eq!(updated.products, vec![OrderProduct::new(seeded_id(), 3)]);
}

#[tokio::test]
async fn list_orders_includes_created_order() {
    let app = app();
    let created = create(&app, SKYWALKER, 1).await;
    let response = send(&app, "GET", "/api/orders", Some(SKYWALKER), None).await;
    assert_eq!(response.status(), StatusCode::OK);
    let orders: Vec<Order> = parse(&response);
    assert!(orders.iter().any(|o| o.id == created.id));
}

#[tokio::test]
async fn invalid_body_returns_bad_request() {
    let app = app();
    let body = json!({ "unexpected": true });
    let response = send(&app, "POST", "/api/orders", Some(SKYWALKER), Some(body)).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn unauthenticated_order_calls_are_rejected() {
    let app = app();
    let body = json!({ "products": [] });
    let response = send(&app, "POST", "/api/orders", None, Some(body)).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let response = send(&app, "GET", "/api/orders", None, None).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
