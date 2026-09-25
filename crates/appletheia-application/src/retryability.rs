use std::error::Error;

/// Reports whether an application error may be retried automatically.
pub trait Retryability: Error + Send + Sync + 'static {
    /// Returns whether the failed operation may succeed when attempted again.
    fn is_retryable(&self) -> bool;
}
