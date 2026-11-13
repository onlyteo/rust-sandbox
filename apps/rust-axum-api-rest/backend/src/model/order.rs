use crate::model::{EnumTypeParseError, enum_type_not_found};
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{AsRefStr, EnumString};
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
    pub products: Vec<OrderProduct>,
}

#[derive(Deserialize)]
pub struct UpdateOrderRequest {
    pub status: OrderStatus,
    pub products: Vec<OrderProduct>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, EnumString, AsRefStr)]
#[strum(
    serialize_all = "SCREAMING_SNAKE_CASE",
    parse_err_fn = enum_type_not_found,
    parse_err_ty = EnumTypeParseError
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Open,
    Completed,
    Cancelled,
}

impl Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
