use std::sync::Arc;

#[derive(Debug)]
pub struct BasicAuthState;

impl BasicAuthState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }
}

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
