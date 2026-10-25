use thiserror::Error;

use super::{ReadModelIncludedError, ReadModelViewError};

#[derive(Debug, Error)]
pub enum ReadModelDocumentError {
    #[error(transparent)]
    ReadModelView(#[from] ReadModelViewError),

    #[error(transparent)]
    Included(#[from] ReadModelIncludedError),
}
