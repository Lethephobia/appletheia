use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::projection::UserFragmentWriterError;

/// Error returned while projecting public user fragments.
#[derive(Debug, Error)]
pub enum UserFragmentProjectorError {
    #[error("user event envelope failed")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("public user fragment writer failed")]
    FragmentWriter(#[from] UserFragmentWriterError),
}
