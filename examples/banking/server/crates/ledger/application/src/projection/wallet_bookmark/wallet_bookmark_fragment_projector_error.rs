use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::projection::WalletBookmarkFragmentWriterError;

/// Error returned while projecting wallet bookmark fragments.
#[derive(Debug, Error)]
pub enum WalletBookmarkFragmentProjectorError {
    #[error("wallet bookmark event envelope failed")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("wallet bookmark fragment writer failed")]
    Writer(#[from] WalletBookmarkFragmentWriterError),
}
