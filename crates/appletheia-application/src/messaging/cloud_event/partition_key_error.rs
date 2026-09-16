use thiserror::Error;

use super::CloudEventAttributeStringError;

#[derive(Debug, Error)]
pub enum PartitionKeyError {
    #[error("value cannot be empty")]
    Empty,
    #[error(transparent)]
    AttributeString(#[from] CloudEventAttributeStringError),
}
