use std::net::TcpListener;

use rust_warp_rest_api_frontend_api::config::{AppBackendConfig, AppConfig};
use rust_warp_rest_api_frontend_api::route;
use rust_warp_rest_api_frontend_api::state::AppState;
use warp::http::{StatusCode, header};
use warp::{Filter, Reply};

const SKYWALKER: &str = "Basic c2t5d2Fsa2VyOnNlY3JldA==";

/// Stub backend that answers with the Authorization header it received.
async fn start_backend() -> String {
    let port = TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let products = warp::path!("api" / "products")
        .and(warp::header::optional::<String>("authorization"))
        .map(|auth: Option<String>| {
            warp::reply::with_status(auth.unwrap_or_default(), StatusCode::ACCEPTED).into_response()
        });
    let server = warp::serve(products).bind(([127, 0, 0, 1], port)).await;
    tokio::spawn(server.run());
    format!("http://127.0.0.1:{port}")
}

async fn frontend_api()
-> impl Filter<Extract = (impl Reply,), Error = std::convert::Infallible> + Clone {
    let backend_url = start_backend().await;
    route::routes(AppState::new(AppConfig {
        backend: AppBackendConfig { url: backend_url },
    }))
}

#[tokio::test]
async fn api_requires_basic_auth() {
    let app = frontend_api().await;
    let response = warp::test::request()
        .path("/api/products")
        .reply(&app)
        .await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn credentials_and_status_are_forwarded_to_backend() {
    let app = frontend_api().await;
    let response = warp::test::request()
        .path("/api/products")
        .header(header::AUTHORIZATION, SKYWALKER)
        .reply(&app)
        .await;
    assert_eq!(response.status(), StatusCode::ACCEPTED);
    assert_eq!(response.body().as_ref(), SKYWALKER.as_bytes());
}

#[tokio::test]
async fn unreachable_backend_returns_bad_gateway() {
    let app = route::routes(AppState::new(AppConfig {
        backend: AppBackendConfig {
            url: "http://127.0.0.1:1".to_string(),
        },
    }));
    let response = warp::test::request()
        .path("/api/products")
        .header(header::AUTHORIZATION, SKYWALKER)
        .reply(&app)
        .await;
    assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
}

#[tokio::test]
async fn index_page_is_public() {
    let app = frontend_api().await;
    let response = warp::test::request().path("/").reply(&app).await;
    assert_eq!(response.status(), StatusCode::OK);
}
