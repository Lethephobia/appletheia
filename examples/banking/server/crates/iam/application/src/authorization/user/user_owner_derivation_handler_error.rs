use appletheia::application::aggregate::SerializedAggregateError;
use banking_iam_domain::UserError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserOwnerDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    User(#[from] UserError),
}
