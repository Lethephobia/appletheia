use appletheia::application::aggregate::SerializedAggregateError;
use banking_iam_domain::TeamError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamOrganizationDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    Team(#[from] TeamError),
}
