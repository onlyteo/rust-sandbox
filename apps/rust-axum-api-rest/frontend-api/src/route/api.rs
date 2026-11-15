use std::sync::Arc;

use auth_handling::axum::basic::middleware::basic_auth;
use auth_handling::model::BasicAuthPrincipal;
use axum::body::{Body, Bytes};
use axum::extract::{Request, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::any;
use axum::{Extension, Router};

use crate::state::AppState;

/// The frontend API exposes exactly the backend API (same paths, methods, bodies and status
/// codes), so there is nothing to translate: every `/api/*` call is proxied as-is. The calls are
/// protected with Basic auth and the caller's credentials are forwarded to the backend.
pub fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/{*path}", any(proxy))
        .route_layer(basic_auth())
}

async fn proxy(
    State(state): State<Arc<AppState>>,
    Extension(principal): Extension<BasicAuthPrincipal>,
    request: Request,
) -> Response {
    let (parts, body) = request.into_parts();
    let path_and_query = parts
        .uri
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or_else(|| parts.uri.path());
    let url = format!("{}{}", state.backend_url, path_and_query);

    let body: Bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(error) => {
            tracing::warn!("Could not read request body: {error}");
            return StatusCode::BAD_REQUEST.into_response();
        }
    };

    let mut backend_request = state
        .client
        .request(parts.method, &url)
        .basic_auth(&principal.username, Some(&principal.password))
        .body(body);
    for name in [header::ACCEPT, header::CONTENT_TYPE] {
        if let Some(value) = parts.headers.get(&name) {
            backend_request = backend_request.header(name, value);
        }
    }

    tracing::debug!("Proxying request to \"{url}\"");
    match backend_request.send().await {
        Ok(response) => {
            let status = response.status();
            let content_type = response.headers().get(header::CONTENT_TYPE).cloned();
            match response.bytes().await {
                Ok(bytes) => respond(status, content_type, bytes),
                Err(error) => bad_gateway(&url, error),
            }
        }
        Err(error) => bad_gateway(&url, error),
    }
}

fn respond(status: StatusCode, content_type: Option<HeaderValue>, bytes: Bytes) -> Response {
    let mut response = Response::new(Body::from(bytes));
    *response.status_mut() = status;
    if let Some(content_type) = content_type {
        response
            .headers_mut()
            .insert(header::CONTENT_TYPE, content_type);
    }
    response
}

fn bad_gateway(url: &str, error: reqwest::Error) -> Response {
    tracing::error!("Request to \"{url}\" failed: {error}");
    StatusCode::BAD_GATEWAY.into_response()
}
