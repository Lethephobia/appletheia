use thiserror::Error;

use super::ReadModelError;

#[derive(Debug, Error)]
pub enum ReadModelIncludedError {
    #[error(transparent)]
    ReadModel(#[from] ReadModelError),
}
