use thiserror::Error;

use super::EventOutboxPollInterval;

#[derive(Debug, Error)]
pub enum EventOutboxPollingOptionsError {
    #[error(
        "base interval must be less than or equal to max interval (base={base:?}, max={max:?})"
    )]
    BaseGreaterThanMax {
        base: EventOutboxPollInterval,
        max: EventOutboxPollInterval,
    },
}
