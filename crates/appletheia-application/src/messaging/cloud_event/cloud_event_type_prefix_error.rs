use thiserror::Error;

use super::CloudEventAttributeStringError;

#[derive(Debug, Error)]
pub enum CloudEventTypePrefixError {
    #[error("value cannot be empty")]
    Empty,
    #[error("type prefix segments cannot be empty")]
    EmptySegment,
    #[error(transparent)]
    AttributeString(#[from] CloudEventAttributeStringError),
}
