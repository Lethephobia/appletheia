use super::ProjectorDefinitionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectorDefinitionBuilderError {
    #[error(transparent)]
    Definition(#[from] ProjectorDefinitionError),
}
