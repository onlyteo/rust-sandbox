use serde_derive::Serialize;

#[derive(Serialize)]
pub struct ErrorBody {
    pub message: String,
}
