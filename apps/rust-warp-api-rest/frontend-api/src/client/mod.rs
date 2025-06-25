use crate::model::greeting::{Greeting, Person};
use warp::hyper::{Body, Client, Request};

pub async fn client_post_greeting(person: Person) -> Result<Greeting, warp::Rejection> {
    let body = serde_json::to_string(&person);
    let client = Client::new();
    let request = Request::post("http://localhost:8081/api/greetings")
        .header("Content-Type", "application/json")
        .body(Body::from(body.unwrap()))
        .unwrap();
    let response = client.request(request).await.unwrap();
    println!("{:#?}", response);
    let body = response.into_body();
    println!("{:#?}", body);
    let greeting = Greeting {
        message: String::from("Hello, World!"),
    };
    Ok(greeting)
}
