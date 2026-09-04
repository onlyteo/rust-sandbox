use crate::client::HttpClient;
use crate::model::Person;
use askama::Template;
use axum::Router;
use axum::extract::Form;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;
use tower_http::services::ServeDir;

pub fn resource_routes() -> Router {
    let serve_dir = ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/static"));
    Router::new()
        .route("/", get(get_home).post(post_home))
        .fallback_service(serve_dir)
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    alert: Option<Alert>,
}

struct Alert {
    variant: &'static str,
    message: String,
}

async fn get_home() -> (StatusCode, Html<String>) {
    (StatusCode::OK, render(None))
}

async fn post_home(Form(person): Form<Person>) -> (StatusCode, Html<String>) {
    let name = person.name.trim();
    if name.is_empty() {
        tracing::warn!("Rejected greeting request with a blank name");
        let alert = Alert {
            variant: "danger",
            message: "Name must not be blank.".to_string(),
        };
        return (StatusCode::BAD_REQUEST, render(Some(alert)));
    }

    tracing::info!("Fetching greeting for \"{name}\"");
    let client = HttpClient::new("http://localhost:8081/api/greetings");
    let person = Person {
        name: name.to_string(),
    };
    let alert = match client.post(person).await {
        Ok(greeting) => Alert {
            variant: "success",
            message: greeting.message,
        },
        Err(error) => {
            tracing::error!("Failed to fetch greeting: {error}");
            Alert {
                variant: "danger",
                message: "Something went wrong, please try again.".to_string(),
            }
        }
    };
    (StatusCode::OK, render(Some(alert)))
}

fn render(alert: Option<Alert>) -> Html<String> {
    let template = IndexTemplate { alert };
    Html(template.render().expect("failed to render index template"))
}
