use std::collections::HashMap;

use crate::model::{Greeting, Person};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};

pub fn api_routes() -> Router {
    Router::new().route("/api/greetings", post(post_greeting).get(get_greetings))
}

async fn post_greeting(person: Json<Person>) -> Json<Greeting> {
    tracing::info!("Returning greeting to \"{}\"", person.name);
    let greeting = Greeting {
        message: format!("Hello, {}!", person.name),
    };
    Json(greeting)
}

async fn get_greetings(
    Query(params): Query<HashMap<String, String>>,
) -> (StatusCode, Json<Vec<Greeting>>) {
    match params.get("name") {
        Some(name) => {
            tracing::info!("Returning greetings");
            let greeting = Greeting {
                message: format!("Hello, {}!", name),
            };
            (StatusCode::OK, Json(vec![greeting]))
        }
        None => todo!(),
    }
}
