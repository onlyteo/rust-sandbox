use axum::http::StatusCode;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S: Serializer>(status: &StatusCode, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_u16(status.as_u16())
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<StatusCode, D::Error> {
    let code = u16::deserialize(deserializer)?;
    StatusCode::from_u16(code).map_err(D::Error::custom)
}
