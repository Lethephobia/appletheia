use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventOutboxPollJitterRatioError {
    #[error("jitter ratio must be between 0.0 and 1.0, got {0}")]
    Invalid(f64),
}
