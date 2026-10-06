use thiserror::Error;

use super::ReadModelResourceKeyError;

#[derive(Debug, Error)]
pub enum ReadModelAttributeKeyError {
    #[error(transparent)]
    ResourceKey(#[from] ReadModelResourceKeyError),
}
