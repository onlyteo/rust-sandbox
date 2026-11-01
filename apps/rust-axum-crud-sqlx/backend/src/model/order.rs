use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Order {
    pub id: Uuid,
    pub username: String,
    pub products: Vec<OrderProduct>,
    pub status: OrderStatus,
    pub timestamp: DateTime<Utc>,
}

impl Order {
    pub fn new(username: String, products: Vec<OrderProduct>) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            products,
            status: OrderStatus::Open,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct OrderProduct {
    pub id: Uuid,
    pub quantity: i32,
}

impl OrderProduct {
    pub fn new(id: Uuid, quantity: i32) -> Self {
        Self { id, quantity }
    }
}

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub username: String,
    pub products: Vec<OrderProduct>,
}

#[derive(Deserialize)]
pub struct UpdateOrderRequest {
    pub status: OrderStatus,
    pub products: Vec<OrderProduct>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Open,
    Completed,
    Cancelled,
}
