use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use error_handling::model::ProblemDetails;
use headers::{Authorization, HeaderMapExt, authorization::Basic};

use crate::{
    axum::basic::model::BasicAuthState,
    model::{BasicAuthPrincipal, BoxedFuture},
};

pub fn basic_auth() -> BasicAuthMiddleware {
    let state = BasicAuthState::new();
    axum::middleware::from_fn_with_state(state, basic_auth_middleware_boxed as BasicAuth)
}

pub type BasicAuthMiddleware = axum::middleware::FromFnLayer<
    BasicAuth,
    Arc<BasicAuthState>,
    (State<Arc<BasicAuthState>>, Request),
>;

type BasicAuth = fn(State<Arc<BasicAuthState>>, Request, Next) -> BoxedFuture<Response>;

fn basic_auth_middleware_boxed(
    state: State<Arc<BasicAuthState>>,
    request: Request,
    next: Next,
) -> BoxedFuture<Response> {
    Box::pin(basic_auth_middleware(state, request, next))
}

async fn basic_auth_middleware(
    State(_state): State<Arc<BasicAuthState>>,
    mut request: Request,
    next: Next,
) -> Response {
    let basic_auth_header = request.headers().typed_get::<Authorization<Basic>>();
    match basic_auth_header {
        Some(basic_auth) => {
            let username = basic_auth.username();
            let password = basic_auth.password();
            let principal = BasicAuthPrincipal::new(username, password);
            request.extensions_mut().insert(principal);
            next.run(request).await
        }
        None => ProblemDetails::unauthorized("Authorization header not found").into_response(),
    }
}
