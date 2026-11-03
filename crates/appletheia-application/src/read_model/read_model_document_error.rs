use thiserror::Error;

use super::{ReadModelIncludedError, ReadModelListDocumentMetaError, ReadModelViewError};

#[derive(Debug, Error)]
pub enum ReadModelDocumentError {
    #[error(transparent)]
    ReadModelView(#[from] ReadModelViewError),

    #[error(transparent)]
    Included(#[from] ReadModelIncludedError),

    #[error(transparent)]
    ListDocumentMeta(#[from] ReadModelListDocumentMetaError),
}
