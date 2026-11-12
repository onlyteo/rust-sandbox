use crate::model::{EnumTypeParseError, enum_type_not_found};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString};
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
    pub products: Vec<OrderProduct>,
}

impl CreateOrderRequest {
    pub fn new(products: Vec<OrderProduct>) -> Self {
        Self { products }
    }
}

#[derive(Serialize, Deserialize)]
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
