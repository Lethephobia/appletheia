use appletheia::application::aggregate::SerializedAggregateError;
use banking_iam_domain::OrganizationMembershipError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationFinanceManagerDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    OrganizationMembership(#[from] OrganizationMembershipError),
}
