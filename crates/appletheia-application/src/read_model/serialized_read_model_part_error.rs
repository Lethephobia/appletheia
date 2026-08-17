use thiserror::Error;

/// Reports an invalid serialized read model part.
#[derive(Debug, Error)]
pub enum SerializedReadModelPartError {
    #[error("read model part must not be null")]
    NullPart,
}
