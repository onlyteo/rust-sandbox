use crate::model::{Greeting, Person};
use axum::Json;

pub async fn post_greeting(person: Json<Person>) -> Json<Greeting> {
    println!("Returning greeting to \"{}\"", person.name);
    let greeting = Greeting {
        message: format!("Hello, {}!", person.name),
    };
    Json(greeting)
}
