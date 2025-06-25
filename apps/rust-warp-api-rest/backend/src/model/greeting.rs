use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Greeting {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Person {
    pub name: String
}
