use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::OrganizationJoinRequestListWriterError;

/// Error returned while projecting organization join request list read models.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestListProjectorError {
    #[error("event envelope failed")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("organization join request list writer failed")]
    Writer(#[from] OrganizationJoinRequestListWriterError),
}
