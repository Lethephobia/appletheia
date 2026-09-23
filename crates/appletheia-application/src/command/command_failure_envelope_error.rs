use thiserror::Error;

use crate::messaging::CommandFailureCloudEventCodecError;

#[derive(Debug, Error)]
pub enum CommandFailureEnvelopeError {
    #[error(transparent)]
    CommandFailureCloudEventCodec(#[from] CommandFailureCloudEventCodecError),
}
