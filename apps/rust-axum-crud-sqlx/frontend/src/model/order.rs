use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub username: String,
    pub products: Vec<OrderProduct>,
    pub status: OrderStatus,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct OrderProduct {
    pub id: Uuid,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub username: String,
    pub products: Vec<OrderProduct>,
}

impl CreateOrderRequest {
    pub fn new(username: String, products: Vec<OrderProduct>) -> Self {
        Self { username, products }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateOrderRequest {
    pub status: OrderStatus,
    pub products: Vec<OrderProduct>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Open,
    Completed,
    Cancelled,
}
