use std::sync::Arc;

use reqwest::{Client, StatusCode, header::AUTHORIZATION};
use uuid::Uuid;

use crate::model::{CreateOrderRequest, Order, OrderProduct, OrderStatus, UpdateOrderRequest};

pub struct OrderService {
    client: Arc<Client>,
    base_url: String,
}

impl OrderService {
    pub fn new(client: Arc<Client>, base_url: String) -> Arc<Self> {
        Arc::new(Self { client, base_url })
    }

    pub async fn get_order_by_id(
        &self,
        username: String,
        id: Uuid,
    ) -> anyhow::Result<Option<Order>> {
        let url = format!("{}/api/orders/{id}", self.base_url);
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

    pub async fn find_orders(&self, username: String) -> anyhow::Result<Vec<Order>> {
        let url = format!("{}/api/orders", self.base_url);
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

    pub async fn create_order(
        &self,
        username: String,
        products: Vec<OrderProduct>,
    ) -> anyhow::Result<Option<Order>> {
        let url = format!("{}/api/orders", self.base_url);
        let response = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Plaintext {}", username))
            .json(&CreateOrderRequest::new(username, products))
            .send()
            .await?;
        match response.status() {
            StatusCode::CREATED => Ok(response.json().await?),
            status => Err(anyhow::anyhow!("Unexpected status code: {status}")),
        }
    }

    pub async fn update_order(
        &self,
        username: String,
        id: Uuid,
        status: OrderStatus,
        products: Vec<OrderProduct>,
    ) -> anyhow::Result<Option<Order>> {
        let url = format!("{}/api/orders/{id}", self.base_url);
        let response = self
            .client
            .put(&url)
            .header(AUTHORIZATION, format!("Plaintext {}", username))
            .json(&UpdateOrderRequest { status, products })
            .send()
            .await?;
        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            status => Err(anyhow::anyhow!("Unexpected status code: {status}")),
        }
    }
}
