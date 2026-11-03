use std::sync::Arc;

use reqwest::{Client, StatusCode, header::AUTHORIZATION};

use crate::model::Product;

pub struct ProductService {
    client: Arc<Client>,
    base_url: String,
}

impl ProductService {
    pub fn new(client: Arc<Client>, base_url: String) -> Arc<Self> {
        Arc::new(Self { client, base_url })
    }

    pub async fn find_products(&self, username: String) -> anyhow::Result<Vec<Product>> {
        let url = format!("{}/api/products", self.base_url);
        let response = self
            .client
            .get(&url)
            .header(AUTHORIZATION, format!("Plaintext {}", username))
            .send()
            .await?;
        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            status => Err(anyhow::anyhow!("Unexpected status code: {status}")),
        }
    }
}
