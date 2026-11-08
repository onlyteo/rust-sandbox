use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use error_handling::model::ProblemDetails;
use headers::{Authorization, HeaderMapExt, authorization::Basic};

use crate::{
    basic::model::{BasicAuthPrincipal, BasicAuthState},
    model::BoxedFuture,
};

pub fn basic_auth_layer() -> BasicAuthMiddlewareLayer {
    let state = BasicAuthState::new();
    axum::middleware::from_fn_with_state(state, basic_auth_middleware_boxed as BasicAuth)
}

pub type BasicAuthMiddlewareLayer = axum::middleware::FromFnLayer<
    BasicAuth,
    Arc<BasicAuthState>,
    (State<Arc<BasicAuthState>>, Request),
>;

type BasicAuth =
    fn(State<Arc<BasicAuthState>>, Request, Next) -> BoxedFuture<Result<Response, ProblemDetails>>;

fn basic_auth_middleware_boxed(
    state: State<Arc<BasicAuthState>>,
    request: Request,
    next: Next,
) -> BoxedFuture<Result<Response, ProblemDetails>> {
    Box::pin(basic_auth_middleware(state, request, next))
}

async fn basic_auth_middleware(
    State(_state): State<Arc<BasicAuthState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, ProblemDetails> {
    let basic_auth_header = request.headers().typed_get::<Authorization<Basic>>();
    match basic_auth_header {
        Some(basic_auth) => {
            let username = basic_auth.username();
            let password = basic_auth.password();
            let principal = BasicAuthPrincipal::new(username, password);
            request.extensions_mut().insert(principal);
            Ok(next.run(request).await)
        }
        None => {
            let instance = request.uri().path().to_string();
            Err(ProblemDetails::unauthorized(
                instance.as_str(),
                "Authorization header not found",
            ))
        }
    }
}
