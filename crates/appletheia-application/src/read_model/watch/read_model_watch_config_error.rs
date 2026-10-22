use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelWatchConfigError {
    #[error("watch lease refresh interval must be shorter than its lease duration")]
    RefreshIntervalTooLong,
}
