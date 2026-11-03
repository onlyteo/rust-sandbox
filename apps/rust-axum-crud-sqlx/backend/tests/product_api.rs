use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use rust_axum_crud_sqlx_backend::model::Product;
use rust_axum_crud_sqlx_backend::route;
use rust_axum_crud_sqlx_backend::service::order::OrderService;
use rust_axum_crud_sqlx_backend::service::product::ProductService;
use rust_axum_crud_sqlx_backend::state::AppState;
use sqlx::PgPool;
use tower::ServiceExt;

#[sqlx::test]
async fn list_products_returns_seeded_rows(pool: PgPool) -> sqlx::Result<()> {
    let order_service = OrderService::new(pool.clone());
    let product_service = ProductService::new(pool.clone());
    let app = route::routes(AppState::new(order_service, product_service));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/products")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let products: Vec<Product> = serde_json::from_slice(&body).unwrap();
    assert_eq!(products.len(), 3);
    assert!(
        products
            .iter()
            .any(|product| product.name == "Roasted Coffee Beans")
    );

    Ok(())
}
