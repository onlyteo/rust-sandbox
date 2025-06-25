use warp::Filter;
use crate::model::greeting;

pub fn post_greetings() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("api"))
        .and(warp::path("greetings"))
        .and(warp::body::json())
        .and_then(handle_greeting)
}

async fn handle_greeting(person: greeting::Person) -> Result<impl warp::Reply, warp::Rejection> {
    let greeting = greeting::Greeting {
        message: format!("Hello, {}!", person.name),
    };
    Ok(warp::reply::json(&greeting))
}