use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use rust_axum_crud_sqlx_backend::model::dto::{Order, OrderProduct, OrderStatus};
use rust_axum_crud_sqlx_backend::route;
use rust_axum_crud_sqlx_backend::state::AppState;
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

const SEEDED_PRODUCT_ID: &str = "6821a470-071b-4cf4-b993-43cc8b2a8827";

async fn parse_order(response: axum::response::Response) -> Order {
    let body = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&body).unwrap()
}

#[sqlx::test]
async fn create_order_returns_open_order(pool: PgPool) -> sqlx::Result<()> {
    let app = route::routes(AppState::new(pool));

    let body = serde_json::json!({
        "products": [{ "id": SEEDED_PRODUCT_ID, "quantity": 2 }]
    });
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/orders")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let order = parse_order(response).await;
    assert_eq!(order.status, OrderStatus::Open);
    assert_eq!(
        order.products,
        vec![OrderProduct {
            id: Uuid::parse_str(SEEDED_PRODUCT_ID).unwrap(),
            quantity: 2
        }]
    );

    Ok(())
}

#[sqlx::test]
async fn get_order_by_id_matches_created_order(pool: PgPool) -> sqlx::Result<()> {
    let app = route::routes(AppState::new(pool));

    let create_body = serde_json::json!({
        "products": [{ "id": SEEDED_PRODUCT_ID, "quantity": 1 }]
    });
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/orders")
                .header("content-type", "application/json")
                .body(Body::from(create_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let created = parse_order(create_response).await;

    let get_response = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/orders/{}", created.id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(get_response.status(), StatusCode::OK);
    let fetched = parse_order(get_response).await;
    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.status, OrderStatus::Open);

    Ok(())
}

#[sqlx::test]
async fn get_order_by_missing_id_returns_not_found(pool: PgPool) -> sqlx::Result<()> {
    let app = route::routes(AppState::new(pool));

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/orders/{}", Uuid::new_v4()))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    Ok(())
}

#[sqlx::test]
async fn update_order_status_is_reflected(pool: PgPool) -> sqlx::Result<()> {
    let app = route::routes(AppState::new(pool));

    let create_body = serde_json::json!({
        "products": [{ "id": SEEDED_PRODUCT_ID, "quantity": 1 }]
    });
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/orders")
                .header("content-type", "application/json")
                .body(Body::from(create_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let created = parse_order(create_response).await;

    let update_body = serde_json::json!({
        "status": "COMPLETED",
        "products": [{ "id": SEEDED_PRODUCT_ID, "quantity": 3 }]
    });
    let update_response = app
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri(format!("/api/orders/{}", created.id))
                .header("content-type", "application/json")
                .body(Body::from(update_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(update_response.status(), StatusCode::OK);
    let updated = parse_order(update_response).await;
    assert_eq!(updated.status, OrderStatus::Completed);
    assert_eq!(
        updated.products,
        vec![OrderProduct {
            id: Uuid::parse_str(SEEDED_PRODUCT_ID).unwrap(),
            quantity: 3
        }]
    );

    Ok(())
}

#[sqlx::test]
async fn list_orders_includes_created_order(pool: PgPool) -> sqlx::Result<()> {
    let app = route::routes(AppState::new(pool));

    let create_body = serde_json::json!({
        "products": [{ "id": SEEDED_PRODUCT_ID, "quantity": 1 }]
    });
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/orders")
                .header("content-type", "application/json")
                .body(Body::from(create_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let created = parse_order(create_response).await;

    let list_response = app
        .oneshot(
            Request::builder()
                .uri("/api/orders")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(list_response.status(), StatusCode::OK);
    let body = list_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let orders: Vec<Order> = serde_json::from_slice(&body).unwrap();
    assert!(orders.iter().any(|order| order.id == created.id));

    Ok(())
}
