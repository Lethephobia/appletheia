use thiserror::Error;

use super::CloudEventAttributeStringError;

#[derive(Debug, Error)]
pub enum CloudEventPartitionKeyError {
    #[error("value cannot be empty")]
    Empty,
    #[error(transparent)]
    AttributeString(#[from] CloudEventAttributeStringError),
}
