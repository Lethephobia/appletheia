use super::SagaInstanceError;
use thiserror::Error;

/// Reports failures in input-scoped saga operations.
#[derive(Debug, Error)]
pub enum SagaContextError {
    #[error(transparent)]
    Instance(#[from] SagaInstanceError),
}
