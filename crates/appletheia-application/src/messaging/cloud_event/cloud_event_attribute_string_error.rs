use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloudEventAttributeStringError {
    #[error("CloudEvents string contains a forbidden Unicode character")]
    InvalidCharacter,
}
