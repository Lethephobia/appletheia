use appletheia::application::event::EventEnvelopeError;
use appletheia::application::read_model::ReadModelFragmentChangeError;
use thiserror::Error;

use crate::projection::OrganizationFragmentWriterError;

/// Error returned while projecting organization fragment.
#[derive(Debug, Error)]
pub enum OrganizationFragmentProjectorError {
    #[error("organization event envelope failed")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("organization fragment writer failed")]
    Writer(#[from] OrganizationFragmentWriterError),

    #[error("organization fragment change failed")]
    FragmentChange(#[from] ReadModelFragmentChangeError),
}
