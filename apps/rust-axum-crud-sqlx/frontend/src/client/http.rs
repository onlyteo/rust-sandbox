use std::sync::Arc;

use anyhow::{Result, anyhow};
use reqwest::{Client, StatusCode};
use uuid::Uuid;

use crate::model::{
    CreateOrderRequest, Order, OrderProduct, OrderStatus, Product, UpdateOrderRequest,
};

#[derive(Clone)]
pub struct HttpClient {
    client: Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: String) -> Arc<Self> {
        Arc::new(Self {
            client: Client::new(),
            base_url,
        })
    }

    pub async fn list_products(&self) -> Result<Vec<Product>> {
        let url = format!("{}/api/products", self.base_url);
        let response = self.client.get(&url).send().await?;
        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            status => Err(anyhow!("Unexpected status code: {status}")),
        }
    }

    pub async fn list_orders(&self) -> Result<Vec<Order>> {
        let url = format!("{}/api/orders", self.base_url);
        let response = self.client.get(&url).send().await?;
        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            status => Err(anyhow!("Unexpected status code: {status}")),
        }
    }

    pub async fn get_order(&self, id: Uuid) -> Result<Order> {
        let url = format!("{}/api/orders/{id}", self.base_url);
        let response = self.client.get(&url).send().await?;
        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            status => Err(anyhow!("Unexpected status code: {status}")),
        }
    }

    pub async fn create_order(
        &self,
        username: String,
        products: Vec<OrderProduct>,
    ) -> Result<Order> {
        let url = format!("{}/api/orders", self.base_url);
        let response = self
            .client
            .post(&url)
            .json(&CreateOrderRequest::new(username, products))
            .send()
            .await?;
        match response.status() {
            StatusCode::CREATED => Ok(response.json().await?),
            status => Err(anyhow!("Unexpected status code: {status}")),
        }
    }

    pub async fn update_order(
        &self,
        id: Uuid,
        status: OrderStatus,
        products: Vec<OrderProduct>,
    ) -> Result<Order> {
        let url = format!("{}/api/orders/{id}", self.base_url);
        let response = self
            .client
            .put(&url)
            .json(&UpdateOrderRequest { status, products })
            .send()
            .await?;
        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            status => Err(anyhow!("Unexpected status code: {status}")),
        }
    }
}
