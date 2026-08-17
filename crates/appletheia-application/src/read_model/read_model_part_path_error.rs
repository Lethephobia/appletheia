use thiserror::Error;

use super::ReadModelPartName;

/// Reports a failure while materializing a read model part path.
#[derive(Debug, Error)]
pub enum ReadModelPartPathError {
    #[error("failed to serialize a read model fragment key for a replacement path")]
    SerializeKey(#[source] serde_json::Error),
    #[error("read model part `{part_name}` is not declared in the read model part tree")]
    UndeclaredPart { part_name: ReadModelPartName },
}
