use thiserror::Error;

use super::CloudEventAttributeStringError;

#[derive(Debug, Error)]
pub enum CloudEventTypeError {
    #[error("value cannot be empty")]
    Empty,
    #[error("event type does not match the configured prefix")]
    PrefixMismatch,
    #[error(transparent)]
    AttributeString(#[from] CloudEventAttributeStringError),
}
