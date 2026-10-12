use thiserror::Error;

use super::{ReadModelIncludedError, ReadModelViewError};

#[derive(Debug, Error)]
pub enum ReadModelListDocumentError {
    #[error(transparent)]
    ReadModelView(#[from] ReadModelViewError),

    #[error(transparent)]
    Included(#[from] ReadModelIncludedError),
}
