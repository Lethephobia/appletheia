use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::OrganizationInternalInfoWriterError;

/// Error returned while projecting organization-internal information.
#[derive(Debug, Error)]
pub enum OrganizationInternalInfoProjectorError {
    #[error("organization event envelope failed")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("organization internal info writer failed")]
    Writer(#[from] OrganizationInternalInfoWriterError),
}
