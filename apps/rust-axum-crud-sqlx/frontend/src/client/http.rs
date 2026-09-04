use crate::model::greeting::{Greeting, Person};
use anyhow::{anyhow, Result};
use reqwest::{Client, StatusCode};

#[derive(Clone)]
pub struct HttpClient {
    client: Client,
    url: &'static str,
}

impl HttpClient {
    pub fn new(url: &'static str) -> Self {
        Self {
            client: Client::new(),
            url,
        }
    }

    pub async fn post(&self, person: Person) -> Result<Greeting> {
        println!("Sending request to \"{}\"", self.url);
        let response = self.client.post(self.url).json(&person).send().await?;
        println!(
            "Received response {} from \"{}\"",
            response.status(),
            self.url
        );
        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            _ => Err(anyhow!("Unexpected status code: {}", response.status())),
        }
    }
}
