use crate::client::http::post_greeting;
use crate::model::greeting::Person;
use warp::Filter;
use anyhow::Result;

pub fn api_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("api"))
        .and(warp::path("greetings"))
        .and(warp::body::json())
        .and_then(get_greeting)
}

async fn get_greeting(person: Person) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Returning greeting to \"{}\"", person.name);
    let result = post_greeting(person).await;
    match result {
        Ok(greeting) => Ok(warp::reply::json(&greeting)),
        Err(_) => Err(warp::reject::reject()), // TODO: Better error handling
    }
}
