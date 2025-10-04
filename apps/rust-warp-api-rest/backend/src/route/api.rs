use crate::model::greeting;
use warp::Filter;

pub fn api_filter(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("api"))
        .and(warp::path("greetings"))
        .and(warp::body::json())
        .and_then(post_greeting)
}

async fn post_greeting(person: greeting::Person) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Returning greeting to \"{}\"", person.name);
    let greeting = greeting::Greeting {
        message: format!("Hello, {}!", person.name),
    };
    Ok(warp::reply::json(&greeting))
}
