use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::PublicOrganizationListWriterError;

/// Error returned while projecting public organization lists.
#[derive(Debug, Error)]
pub enum PublicOrganizationListProjectorError {
    #[error("organization event envelope failed")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("public organization list writer failed")]
    Writer(#[from] PublicOrganizationListWriterError),
}
