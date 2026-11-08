use std::sync::Arc;

use reqwest::{Client, StatusCode};

use crate::model::{BootstrapPrincipal, Product};

pub struct ProductService {
    client: Arc<Client>,
    base_url: String,
}

impl ProductService {
    pub fn new(client: Arc<Client>, base_url: String) -> Arc<Self> {
        Arc::new(Self { client, base_url })
    }

    pub async fn find_products(
        &self,
        principal: Arc<BootstrapPrincipal>,
    ) -> anyhow::Result<Vec<Product>> {
        let url = format!("{}/api/products", self.base_url);
        let response = self
            .client
            .get(&url)
            .basic_auth(principal.username.clone(), Some(principal.password.clone()))
            .send()
            .await?;
        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            status => Err(anyhow::anyhow!("Unexpected status code: {status}")),
        }
    }
}
