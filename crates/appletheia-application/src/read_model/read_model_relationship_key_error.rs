use thiserror::Error;

use super::ReadModelResourceKeyError;

#[derive(Debug, Error)]
pub enum ReadModelRelationshipKeyError {
    #[error(transparent)]
    ResourceKey(#[from] ReadModelResourceKeyError),
}
