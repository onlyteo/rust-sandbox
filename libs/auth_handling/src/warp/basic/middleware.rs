use crate::model::BasicAuthPrincipal;
use error_handling::warp::model::ErrorResponse;
use headers::{Authorization, HeaderMapExt, authorization::Basic};
use warp::filters::header;
use warp::filters::path::FullPath;
use warp::http::HeaderMap;
use warp::{Filter, Rejection, path, reject};

pub fn basic_auth() -> impl Filter<Extract = (BasicAuthPrincipal, String), Error = Rejection> + Clone
{
    path::full()
        .and(header::headers_cloned())
        .and_then(|path: FullPath, headers: HeaderMap| async move {
            let instance = path.as_str().to_string();
            match headers.typed_get::<Authorization<Basic>>() {
                Some(basic) => Ok((
                    BasicAuthPrincipal::new(basic.username(), basic.password()),
                    instance,
                )),
                None => Err(reject::custom(ErrorResponse::missing_auth_header(
                    &instance,
                ))),
            }
        })
        .untuple_one()
}
