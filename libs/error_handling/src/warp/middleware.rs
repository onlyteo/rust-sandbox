use std::convert::Infallible;

use warp::{
    filters::body::BodyDeserializeError,
    reject::{MethodNotAllowed, Rejection, UnsupportedMediaType},
    reply::Response,
};

use crate::{model::ProblemDetails, warp::model::ErrorResponse};

pub async fn error_handling(rejection: Rejection) -> Result<Response, Infallible> {
    let problem = if let Some(ErrorResponse(problem)) = rejection.find::<ErrorResponse>() {
        problem.clone()
    } else if rejection.is_not_found() {
        ProblemDetails::not_found("Resource not found")
    } else if let Some(error) = rejection.find::<BodyDeserializeError>() {
        ProblemDetails::bad_request(&error.to_string())
    } else if rejection.find::<MethodNotAllowed>().is_some() {
        ProblemDetails::method_not_allowed("Method not allowed")
    } else if rejection.find::<UnsupportedMediaType>().is_some() {
        ProblemDetails::unsupported_media_type("Content-Type must be application/json")
    } else {
        tracing::error!("Unhandled error: {rejection:?}");
        ProblemDetails::internal_server_error("An internal error occurred")
    };

    let response: Response = problem.into();
    Ok(response)
}
