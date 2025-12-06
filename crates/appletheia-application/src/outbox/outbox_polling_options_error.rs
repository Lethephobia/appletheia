use thiserror::Error;

use super::OutboxPollInterval;

#[derive(Debug, Error)]
pub enum OutboxPollingOptionsError {
    #[error(
        "base interval must be less than or equal to max interval (base={base:?}, max={max:?})"
    )]
    BaseGreaterThanMax {
        base: OutboxPollInterval,
        max: OutboxPollInterval,
    },
}
