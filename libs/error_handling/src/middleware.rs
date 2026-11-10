use axum::{
    body::to_bytes,
    extract::Request,
    http::{HeaderMap, header},
    middleware::{FromFnLayer, Next, from_fn},
    response::{IntoResponse, Response},
};

use crate::model::{BoxedFuture, ProblemDetails};

const MAX_REJECTION_BODY: usize = 8 * 1024;

pub fn error_handling_layer() -> ErrorHandlingMiddlewareLayer {
    from_fn(error_handling_middleware_boxed as ErrorHandling)
}

pub type ErrorHandlingMiddlewareLayer = FromFnLayer<ErrorHandling, (), (Request,)>;

type ErrorHandling = fn(Request, Next) -> BoxedFuture<Response>;

fn error_handling_middleware_boxed(request: Request, next: Next) -> BoxedFuture<Response> {
    Box::pin(error_handling_middleware(request, next))
}

async fn error_handling_middleware(request: Request, next: Next) -> Response {
    let instance = request.uri().path().to_string();
    let mut response = next.run(request).await;

    if let Some(mut problem) = response.extensions_mut().remove::<ProblemDetails>() {
        problem.instance = instance;
        return render(problem, response.headers());
    }

    let status = response.status();
    let is_problem = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.starts_with("application/problem+json"));
    if (status.is_client_error() || status.is_server_error()) && !is_problem {
        let (parts, body) = response.into_parts();
        let detail = match to_bytes(body, MAX_REJECTION_BODY).await {
            Ok(bytes) if status.is_client_error() => String::from_utf8_lossy(&bytes).into_owned(),
            _ => String::new(),
        };
        let reason = status.canonical_reason().unwrap_or("Error");
        let mut problem = ProblemDetails::new(
            &format!("urn:http:{}", reason.to_lowercase().replace(' ', "-")),
            reason,
            status,
            &detail,
        );
        problem.instance = instance;
        return render(problem, &parts.headers);
    }
    response
}

fn render(problem: ProblemDetails, original: &HeaderMap) -> Response {
    let mut response = problem.into_response();
    for (name, value) in original {
        if name != header::CONTENT_TYPE && name != header::CONTENT_LENGTH {
            response.headers_mut().append(name.clone(), value.clone());
        }
    }
    response
}
