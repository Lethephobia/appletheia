use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationInvitationAcceptedSagaError {
    #[error("unexpected organization invitation accepted saga event")]
    UnexpectedEvent,
}
