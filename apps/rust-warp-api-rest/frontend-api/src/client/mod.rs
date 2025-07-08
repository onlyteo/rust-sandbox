use crate::model::greeting::{Greeting, Person};
use serde::Deserialize;
use warp::hyper::body::to_bytes;
use warp::hyper::{Body, Client, Request};
use warp::Buf;

pub async fn client_post_greeting(person: Person) -> Result<Greeting, warp::Rejection> {
    let body = serde_json::to_string(&person);
    let client = Client::new();
    let request = Request::post("http://localhost:8081/api/greetings")
        .header("Content-Type", "application/json")
        .body(Body::from(body.unwrap()))
        .unwrap();
    let response = client.request(request).await.unwrap();
    let body_bytes = to_bytes(response.into_body()).await;
    let mut deserializer = serde_json::Deserializer::from_reader(body_bytes.unwrap().reader());
    let greeting = Greeting::deserialize(&mut deserializer);
    Ok(greeting.unwrap())
}
