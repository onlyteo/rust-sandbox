use crate::model::{Greeting, Person};
use axum::routing::post;
use axum::{Json, Router};

pub fn api_routes() -> Router {
    Router::new().route("/api/greetings", post(post_greeting))
}

async fn post_greeting(person: Json<Person>) -> Json<Greeting> {
    tracing::info!("Returning greeting to \"{}\"", person.name);
    let greeting = Greeting {
        message: format!("Hello, {}!", person.name),
    };
    Json(greeting)
}
