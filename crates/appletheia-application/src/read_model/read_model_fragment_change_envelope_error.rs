use thiserror::Error;

use super::ReadModelFragmentChangeError;

/// Reports an invalid durable source-fragment change envelope.
#[derive(Debug, Error)]
pub enum ReadModelFragmentChangeEnvelopeError {
    #[error("a source-fragment change envelope must contain at least one change")]
    EmptyChanges,

    #[error("read model fragment change is invalid")]
    FragmentChange(#[from] ReadModelFragmentChangeError),

    #[error("a source-fragment change does not match the envelope partition")]
    PartitionMismatch,
}
