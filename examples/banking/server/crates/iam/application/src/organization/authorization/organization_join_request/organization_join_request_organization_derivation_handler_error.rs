use appletheia::application::aggregate::SerializedAggregateError;
use banking_iam_domain::OrganizationJoinRequestError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationJoinRequestOrganizationDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    OrganizationJoinRequest(#[from] OrganizationJoinRequestError),
}
