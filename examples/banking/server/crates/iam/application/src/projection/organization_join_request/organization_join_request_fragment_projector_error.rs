use appletheia::application::event::EventEnvelopeError;
use appletheia::application::read_model::ReadModelFragmentChangeError;
use thiserror::Error;

use crate::projection::OrganizationJoinRequestFragmentWriterError;

/// Error returned while projecting organization join request fragments.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestFragmentProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    Writer(#[from] OrganizationJoinRequestFragmentWriterError),

    #[error(transparent)]
    FragmentChange(#[from] ReadModelFragmentChangeError),
}
