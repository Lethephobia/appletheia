use thiserror::Error;

use super::{ReadModelError, ReadModelIncludedError};

#[derive(Debug, Error)]
pub enum ReadModelDocumentError {
    #[error(transparent)]
    ReadModel(#[from] ReadModelError),

    #[error(transparent)]
    Included(#[from] ReadModelIncludedError),
}
