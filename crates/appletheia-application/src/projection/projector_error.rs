use super::ProjectorDefinitionBuilderError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectorError {
    #[error(transparent)]
    DefinitionBuilder(#[from] ProjectorDefinitionBuilderError),
}
