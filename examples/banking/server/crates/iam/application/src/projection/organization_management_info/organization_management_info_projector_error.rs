use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::OrganizationManagementInfoWriterError;

/// Error returned while projecting organization-management information.
#[derive(Debug, Error)]
pub enum OrganizationManagementInfoProjectorError {
    #[error("organization or user event envelope failed")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("organization management info writer failed")]
    Writer(#[from] OrganizationManagementInfoWriterError),
}
