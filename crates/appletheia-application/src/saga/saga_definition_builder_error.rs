use super::SagaDefinitionError;
use thiserror::Error;

/// Reports failures while building a validated saga definition.
#[derive(Debug, Error)]
pub enum SagaDefinitionBuilderError {
    #[error(transparent)]
    Definition(#[from] SagaDefinitionError),
}
