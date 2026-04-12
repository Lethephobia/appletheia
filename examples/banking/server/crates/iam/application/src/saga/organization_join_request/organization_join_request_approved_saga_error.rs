use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationJoinRequestApprovedSagaError {
    #[error("unexpected organization join request approved saga event")]
    UnexpectedEvent,
}
