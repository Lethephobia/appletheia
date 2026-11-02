use super::ReadModelWindowError;

#[derive(Debug, thiserror::Error)]
pub enum ReadModelListDocumentMetaError {
    #[error(transparent)]
    Window(#[from] ReadModelWindowError),
}
