use thiserror::Error;

use super::{ReadModelError, ReadModelIncludedError};

#[derive(Debug, Error)]
pub enum ReadModelListDocumentError {
    #[error(transparent)]
    ReadModel(#[from] ReadModelError),

    #[error(transparent)]
    Included(#[from] ReadModelIncludedError),
}
