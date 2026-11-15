use serde::Deserialize;

#[derive(Deserialize)]
pub struct BasicAuthConfig {
    pub username: String,
    pub password: String,
}
