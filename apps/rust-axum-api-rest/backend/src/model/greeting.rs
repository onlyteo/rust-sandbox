use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Greeting {
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Person {
    pub name: String,
}
