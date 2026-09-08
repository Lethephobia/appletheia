use thiserror::Error;

/// Reports definition validation failures.
#[derive(Debug, Error)]
pub enum SagaDefinitionError {
    #[error("duplicate event and step route")]
    DuplicateEventRoute,
    #[error("duplicate command failure step")]
    DuplicateCommandFailureRoute,
}
