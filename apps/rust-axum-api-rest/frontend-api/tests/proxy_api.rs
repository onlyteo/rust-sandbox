use axum::Router;
use axum::http::{HeaderMap, StatusCode, header};
use axum::routing::get;
use rust_axum_rest_api_frontend_api::config::{AppBackendConfig, AppConfig};
use rust_axum_rest_api_frontend_api::route;
use rust_axum_rest_api_frontend_api::state::AppState;
use tokio::net::TcpListener;

async fn serve(router: Router) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
    format!("http://{address}")
}

/// Stub backend that answers with the Authorization header it received.
async fn start_frontend_api() -> String {
    let backend = Router::new().route(
        "/api/products",
        get(|headers: HeaderMap| async move {
            let auth = headers
                .get(header::AUTHORIZATION)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_string();
            (StatusCode::ACCEPTED, auth)
        }),
    );
    let backend_url = serve(backend).await;
    let state = AppState::new(AppConfig {
        backend: AppBackendConfig { url: backend_url },
    });
    serve(route::routes(state)).await
}

#[tokio::test]
async fn api_requires_basic_auth() {
    let url = start_frontend_api().await;
    let response = reqwest::get(format!("{url}/api/products")).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn credentials_and_status_are_forwarded_to_backend() {
    let url = start_frontend_api().await;
    let response = reqwest::Client::new()
        .get(format!("{url}/api/products"))
        .basic_auth("skywalker", Some("secret"))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::ACCEPTED);
    // base64("skywalker:secret")
    assert_eq!(
        response.text().await.unwrap(),
        "Basic c2t5d2Fsa2VyOnNlY3JldA=="
    );
}

#[tokio::test]
async fn index_page_is_public() {
    let url = start_frontend_api().await;
    let response = reqwest::get(format!("{url}/")).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
