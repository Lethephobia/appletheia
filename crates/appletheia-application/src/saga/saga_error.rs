use super::SagaDefinitionBuilderError;
use thiserror::Error;

/// Reports failures while defining an application saga.
#[derive(Debug, Error)]
pub enum SagaError {
    #[error(transparent)]
    DefinitionBuilder(#[from] SagaDefinitionBuilderError),
}
