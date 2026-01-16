use super::{EventOutboxMaxAttempts, EventOutboxRetryDelay};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct EventOutboxRetryOptions {
    pub backoff: EventOutboxRetryDelay,
    pub max_attempts: EventOutboxMaxAttempts,
}
