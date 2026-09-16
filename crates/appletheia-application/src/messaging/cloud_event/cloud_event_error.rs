use thiserror::Error;

use super::CloudEventExtensionsError;

#[derive(Debug, Error)]
pub enum CloudEventError {
    #[error("payload representation does not match its content type")]
    IncompatibleDataContentType,
    #[error(transparent)]
    Extensions(#[from] CloudEventExtensionsError),
}
