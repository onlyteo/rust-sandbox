use crate::client::client_post_greeting;
use crate::model::greeting::Person;
use warp::Filter;

pub fn api_filter(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("api"))
        .and(warp::path("greetings"))
        .and(warp::body::json())
        .and_then(get_greeting)
}

async fn get_greeting(person: Person) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Returning greeting to \"{}\"", person.name);
    let future = client_post_greeting(person);
    Ok(warp::reply::json(&future.await?))
}
