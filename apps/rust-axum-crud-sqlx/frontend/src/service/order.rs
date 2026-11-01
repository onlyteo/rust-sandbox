use std::sync::Arc;

use reqwest::{Client, StatusCode};
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

    pub async fn get_order_by_id(&self, id: Uuid) -> anyhow::Result<Option<Order>> {
        let url = format!("{}/api/orders/{id}", self.base_url);
        let response = self.client.get(&url).send().await?;
        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            status => Err(anyhow::anyhow!("Unexpected status code: {status}")),
        }
    }

    pub async fn find_orders(&self) -> anyhow::Result<Vec<Order>> {
        let url = format!("{}/api/orders", self.base_url);
        let response = self.client.get(&url).send().await?;
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
        id: Uuid,
        status: OrderStatus,
        products: Vec<OrderProduct>,
    ) -> anyhow::Result<Option<Order>> {
        let url = format!("{}/api/orders/{id}", self.base_url);
        let response = self
            .client
            .put(&url)
            .json(&UpdateOrderRequest { status, products })
            .send()
            .await?;
        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            status => Err(anyhow::anyhow!("Unexpected status code: {status}")),
        }
    }
}
