use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloudEventTypePrefixError {
    #[error("value cannot be empty")]
    Empty,
    #[error("type prefix segments cannot be empty")]
    EmptySegment,
    #[error("type prefix contains a prohibited character")]
    InvalidCharacter,
}
