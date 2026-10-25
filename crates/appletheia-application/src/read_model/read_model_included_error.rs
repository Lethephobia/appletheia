use thiserror::Error;

use super::ReadModelViewError;

#[derive(Debug, Error)]
pub enum ReadModelIncludedError {
    #[error(transparent)]
    ReadModelView(#[from] ReadModelViewError),
}
