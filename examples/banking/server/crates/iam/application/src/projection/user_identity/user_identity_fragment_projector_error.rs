use appletheia::application::event::EventEnvelopeError;
use appletheia::application::read_model::ReadModelFragmentChangeError;
use thiserror::Error;

use crate::projection::UserIdentityFragmentWriterError;

/// Error returned while projecting user identity fragments.
#[derive(Debug, Error)]
pub enum UserIdentityFragmentProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    Writer(#[from] UserIdentityFragmentWriterError),

    #[error(transparent)]
    FragmentChange(#[from] ReadModelFragmentChangeError),
}
