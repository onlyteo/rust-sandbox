use std::sync::Arc;

use auth_handling::model::BasicAuthPrincipal;
use auth_handling::warp::basic::middleware::basic_auth;
use warp::http::{HeaderMap, Method, Response, StatusCode, header};
use warp::hyper::body::Bytes;
use warp::path::FullPath;
use warp::{Filter, Rejection, Reply};

use crate::state::AppState;

fn with_state(
    state: Arc<AppState>,
) -> impl Filter<Extract = (Arc<AppState>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

fn raw_query() -> impl Filter<Extract = (String,), Error = std::convert::Infallible> + Clone {
    warp::query::raw().or(warp::any().map(String::new)).unify()
}

/// The frontend API exposes exactly the backend API (same paths, methods, bodies and status
/// codes), so there is nothing to translate: every `/api/*` call is proxied as-is. The calls are
/// protected with Basic auth and the caller's credentials are forwarded to the backend.
pub fn api_filter(
    state: Arc<AppState>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("api")
        .and(basic_auth())
        .and(with_state(state))
        .and(warp::method())
        .and(warp::path::full())
        .and(raw_query())
        .and(warp::header::headers_cloned())
        .and(warp::body::bytes())
        .and_then(proxy)
}

#[allow(clippy::too_many_arguments)]
async fn proxy(
    principal: BasicAuthPrincipal,
    _instance: String,
    state: Arc<AppState>,
    method: Method,
    path: FullPath,
    query: String,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response<Bytes>, Rejection> {
    let mut url = format!("{}{}", state.backend_url, path.as_str());
    if !query.is_empty() {
        url.push('?');
        url.push_str(&query);
    }

    let mut backend_request = state
        .client
        .request(method, &url)
        .basic_auth(&principal.username, Some(&principal.password))
        .body(body);
    for name in [header::ACCEPT, header::CONTENT_TYPE] {
        if let Some(value) = headers.get(&name) {
            backend_request = backend_request.header(name, value);
        }
    }

    tracing::debug!("Proxying request to \"{url}\"");
    let response = match backend_request.send().await {
        Ok(response) => response,
        Err(error) => return Ok(bad_gateway(&url, error)),
    };
    let status = response.status();
    let content_type = response.headers().get(header::CONTENT_TYPE).cloned();
    let bytes = match response.bytes().await {
        Ok(bytes) => bytes,
        Err(error) => return Ok(bad_gateway(&url, error)),
    };

    let mut reply = Response::new(bytes);
    *reply.status_mut() = status;
    if let Some(content_type) = content_type {
        reply
            .headers_mut()
            .insert(header::CONTENT_TYPE, content_type);
    }
    Ok(reply)
}

fn bad_gateway(url: &str, error: reqwest::Error) -> Response<Bytes> {
    tracing::error!("Request to \"{url}\" failed: {error}");
    let mut reply = Response::new(Bytes::new());
    *reply.status_mut() = StatusCode::BAD_GATEWAY;
    reply
}
