use std::sync::Arc;

#[derive(Debug)]
pub struct BasicAuthState;

impl BasicAuthState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }
}
