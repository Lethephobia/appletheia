use thiserror::Error;

use super::CloudEventAttributeStringError;

#[derive(Debug, Error)]
pub enum CloudEventDataContentTypeError {
    #[error("content type cannot be empty")]
    Empty,
    #[error(transparent)]
    AttributeString(#[from] CloudEventAttributeStringError),
}
