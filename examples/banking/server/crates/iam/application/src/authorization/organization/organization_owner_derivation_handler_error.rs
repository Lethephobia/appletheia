use appletheia::application::aggregate::SerializedAggregateError;
use banking_iam_domain::OrganizationError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationOwnerDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    Organization(#[from] OrganizationError),
}
