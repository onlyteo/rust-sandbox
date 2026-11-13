mod common;

use axum::http::{Method, StatusCode};
use common::*;
use rust_axum_rest_api_backend::model::{Order, OrderProduct, OrderStatus};
use serde_json::json;
use uuid::Uuid;

fn seeded_id() -> Uuid {
    Uuid::parse_str(SEEDED_PRODUCT_ID).unwrap()
}

async fn create(app: &axum::Router, auth: &str, quantity: i32) -> Order {
    let body = json!({ "products": [{ "id": SEEDED_PRODUCT_ID, "quantity": quantity }] });
    let response = send(app, Method::POST, "/api/orders", auth, Some(body)).await;
    assert_eq!(response.status(), StatusCode::CREATED);
    parse(response).await
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
    let response = send(&app, Method::POST, "/api/orders", SKYWALKER, Some(body)).await;
    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn new_open_order_allowed_after_completion() {
    let app = app();
    let order = create(&app, SKYWALKER, 1).await;
    let body =
        json!({ "status": "COMPLETED", "products": [{ "id": SEEDED_PRODUCT_ID, "quantity": 1 }] });
    let uri = format!("/api/orders/{}", order.id);
    send(&app, Method::PUT, &uri, SKYWALKER, Some(body)).await;
    create(&app, SKYWALKER, 1).await;
}

#[tokio::test]
async fn get_order_by_id_matches_created_order() {
    let app = app();
    let created = create(&app, SKYWALKER, 1).await;
    let response = send(
        &app,
        Method::GET,
        &format!("/api/orders/{}", created.id),
        SKYWALKER,
        None,
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let fetched: Order = parse(response).await;
    assert_eq!(fetched.id, created.id);
}

#[tokio::test]
async fn get_missing_order_returns_not_found() {
    let app = app();
    let uri = format!("/api/orders/{}", Uuid::new_v4());
    let response = send(&app, Method::GET, &uri, SKYWALKER, None).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn orders_are_scoped_to_user() {
    let app = app();
    let created = create(&app, SKYWALKER, 1).await;

    let uri = format!("/api/orders/{}", created.id);
    let response = send(&app, Method::GET, &uri, VADER, None).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = json!({ "status": "CANCELLED", "products": [] });
    let response = send(&app, Method::PUT, &uri, VADER, Some(body)).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let response = send(&app, Method::GET, "/api/orders", VADER, None).await;
    let orders: Vec<Order> = parse(response).await;
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
    let response = send(&app, Method::POST, "/api/orders", SKYWALKER, Some(body)).await;
    let created: Order = parse(response).await;
    assert_eq!(created.products.len(), 2);

    let body =
        json!({ "status": "COMPLETED", "products": [{ "id": SEEDED_PRODUCT_ID, "quantity": 3 }] });
    let uri = format!("/api/orders/{}", created.id);
    let response = send(&app, Method::PUT, &uri, SKYWALKER, Some(body)).await;
    assert_eq!(response.status(), StatusCode::OK);
    let updated: Order = parse(response).await;
    assert_eq!(updated.status, OrderStatus::Completed);
    assert_eq!(updated.products, vec![OrderProduct::new(seeded_id(), 3)]);
}

#[tokio::test]
async fn list_orders_includes_created_order() {
    let app = app();
    let created = create(&app, SKYWALKER, 1).await;
    let response = send(&app, Method::GET, "/api/orders", SKYWALKER, None).await;
    assert_eq!(response.status(), StatusCode::OK);
    let orders: Vec<Order> = parse(response).await;
    assert!(orders.iter().any(|o| o.id == created.id));
}
