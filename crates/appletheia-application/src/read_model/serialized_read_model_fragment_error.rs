use thiserror::Error;

/// Reports an invalid serialized read model fragment.
#[derive(Debug, Error)]
pub enum SerializedReadModelFragmentError {
    #[error("read model fragment must not be null")]
    NullFragment,
}
