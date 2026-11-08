use std::sync::Arc;

use reqwest::{Client, StatusCode};
use uuid::Uuid;

use crate::model::{
    BootstrapPrincipal, CreateOrderRequest, Order, OrderProduct, OrderStatus, UpdateOrderRequest,
};

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
        principal: Arc<BootstrapPrincipal>,
        id: Uuid,
    ) -> anyhow::Result<Option<Order>> {
        let url = format!("{}/api/orders/{id}", self.base_url);
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

    pub async fn find_orders(
        &self,
        principal: Arc<BootstrapPrincipal>,
    ) -> anyhow::Result<Vec<Order>> {
        let url = format!("{}/api/orders", self.base_url);
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

    pub async fn create_order(
        &self,
        principal: Arc<BootstrapPrincipal>,
        products: Vec<OrderProduct>,
    ) -> anyhow::Result<Option<Order>> {
        let url = format!("{}/api/orders", self.base_url);
        let response = self
            .client
            .post(&url)
            .basic_auth(principal.username.clone(), Some(principal.password.clone()))
            .json(&CreateOrderRequest::new(
                principal.username.clone(),
                products,
            ))
            .send()
            .await?;
        match response.status() {
            StatusCode::CREATED => Ok(response.json().await?),
            status => Err(anyhow::anyhow!("Unexpected status code: {status}")),
        }
    }

    pub async fn update_order(
        &self,
        principal: Arc<BootstrapPrincipal>,
        id: Uuid,
        status: OrderStatus,
        products: Vec<OrderProduct>,
    ) -> anyhow::Result<Option<Order>> {
        let url = format!("{}/api/orders/{id}", self.base_url);
        let response = self
            .client
            .put(&url)
            .basic_auth(principal.username.clone(), Some(principal.password.clone()))
            .json(&UpdateOrderRequest { status, products })
            .send()
            .await?;
        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            status => Err(anyhow::anyhow!("Unexpected status code: {status}")),
        }
    }
}
