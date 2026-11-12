pub mod alert;
pub mod cart;
pub mod order;
pub mod principal;
pub mod product;

pub use alert::*;
pub use cart::*;
pub use order::*;
pub use principal::*;
pub use product::*;

pub fn enum_type_not_found(type_: &str) -> EnumTypeParseError {
    EnumTypeParseError::UkjentType(type_.to_string())
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum EnumTypeParseError {
    #[error("Ukjent enum: {0}")]
    UkjentType(String),
}
