use crate::model::core::Return;
use crate::model::greeting::{Greeting, Person};
use http_body_util::BodyExt;
use hyper::{body::Buf, Method, Request, Uri};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

pub async fn post_greeting(person: Person) -> Return<Greeting> {
    let url = "http://localhost:8081/api/greetings".parse::<Uri>()?;
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(8081);
    let address = format!("{}:{}", host, port);
    let stream = TcpStream::connect(address.clone()).await?;
    let io = TokioIo::new(stream);
    let (mut sender, connection) = hyper::client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move {
        if let Err(err) = connection.await {
            println!("Connection failed: {:?}", err);
        }
    });
    let authority = url.authority().unwrap().clone();
    let request_body = serde_json::to_string(&person)?;
    let request = Request::builder()
        .uri(url)
        .method(Method::POST)
        .header(hyper::header::HOST, authority.as_str())
        .body(request_body)?;
    println!("Sending request to \"{}\"", address.clone());
    let response = sender.send_request(request).await?;
    println!("Received response from \"{}\"", address.clone());
    let response_body = response.collect().await?.aggregate();
    let greeting = serde_json::from_reader(response_body.reader())?;
    Ok(greeting)
}
