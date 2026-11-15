use warp::{
    http::{StatusCode, header},
    reject::Reject,
    reply::{self, Reply, Response},
};

use crate::model::ProblemDetails;

#[derive(Debug)]
pub struct ErrorResponse(pub ProblemDetails);

impl ErrorResponse {
    pub fn new(mut problem: ProblemDetails, instance: &str) -> Self {
        problem.instance = instance.to_string();
        Self(problem)
    }

    pub fn missing_auth_header(instance: &str) -> Self {
        let mut problem = ProblemDetails::unauthorized("Authorization header not found");
        problem.instance = instance.to_string();
        Self(problem)
    }
}

impl Reject for ErrorResponse {}

impl Into<Response> for ProblemDetails {
    fn into(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap();
        let reply = reply::with_status(warp::reply::json(&self), status);
        reply::with_header(reply, header::CONTENT_TYPE, "application/problem+json").into_response()
    }
}
