use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub packing: String,
    pub price: String,
    pub stock: i32,
}
