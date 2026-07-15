use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::WalletBookmarkListWriterError;

/// Error returned while projecting wallet bookmark lists.
#[derive(Debug, Error)]
pub enum WalletBookmarkListProjectorError {
    #[error("wallet bookmark event envelope failed")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("wallet bookmark list writer failed")]
    Writer(#[from] WalletBookmarkListWriterError),
}
