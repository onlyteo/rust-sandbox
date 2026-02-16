use crate::client::HttpClient;
use crate::model::Greeting;
use crate::model::Person;
use axum::Json;

pub async fn post_greeting(person: Json<Person>) -> Json<Greeting> {
    println!("Returning greeting to \"{}\"", person.name);
    let client = HttpClient::new("http://localhost:8081/api/greetings");
    let greeting = client.post(person.0).await.unwrap();
    Json(greeting)
}
