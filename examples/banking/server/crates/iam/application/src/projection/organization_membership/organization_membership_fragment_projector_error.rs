use appletheia::application::event::EventEnvelopeError;
use appletheia::application::read_model::ReadModelFragmentChangeError;
use thiserror::Error;

use crate::projection::OrganizationMembershipFragmentWriterError;

/// Error returned while projecting organization membership fragments.
#[derive(Debug, Error)]
pub enum OrganizationMembershipFragmentProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    Writer(#[from] OrganizationMembershipFragmentWriterError),

    #[error(transparent)]
    FragmentChange(#[from] ReadModelFragmentChangeError),
}
