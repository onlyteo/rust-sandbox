use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct BasicAuthPrincipal {
    pub username: String,
    pub password: String,
}

impl BasicAuthPrincipal {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

pub type BoxedFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;
