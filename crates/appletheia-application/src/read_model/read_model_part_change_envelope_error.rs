use thiserror::Error;

/// Reports an invalid client-facing part-change envelope.
#[derive(Debug, Error)]
pub enum ReadModelPartChangeEnvelopeError {
    #[error("a part-change envelope must contain at least one change")]
    EmptyChanges,
}
