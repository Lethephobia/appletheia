use thiserror::Error;

use crate::read_model::list::{
    SerializedReadModelListCoverageError, SerializedReadModelListQueryError,
};

/// Reports an application-defined list watch that cannot be deserialized.
#[derive(Debug, Error)]
pub enum ReadModelTypedListWatchError {
    #[error("read model list query is invalid: {0}")]
    Query(#[from] SerializedReadModelListQueryError),
    #[error("read model list coverage is invalid: {0}")]
    Coverage(#[from] SerializedReadModelListCoverageError),
}
