use appletheia::application::aggregate::SerializedAggregateError;
use banking_iam_domain::OrganizationInvitationError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationInvitationInviteeDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    OrganizationInvitation(#[from] OrganizationInvitationError),
}
