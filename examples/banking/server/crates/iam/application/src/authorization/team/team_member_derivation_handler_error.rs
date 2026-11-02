use appletheia::application::aggregate::SerializedAggregateError;
use banking_iam_domain::TeamMembershipError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamMemberDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    TeamMembership(#[from] TeamMembershipError),
}
