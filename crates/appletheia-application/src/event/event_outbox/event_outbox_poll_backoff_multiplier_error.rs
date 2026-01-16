use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventOutboxPollBackoffMultiplierError {
    #[error("backoff multiplier must be finite and >= 1.0, got {0}")]
    Invalid(f64),
}
