pub struct Alert {
    pub variant: &'static str,
    pub message: String,
}

impl Alert {
    pub fn danger(message: impl Into<String>) -> Self {
        Self {
            variant: "danger",
            message: message.into(),
        }
    }
}
