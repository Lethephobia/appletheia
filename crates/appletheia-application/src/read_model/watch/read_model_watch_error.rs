use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelWatchError {
    #[error("a watch must contain at least one selector")]
    EmptySelectors,
}
