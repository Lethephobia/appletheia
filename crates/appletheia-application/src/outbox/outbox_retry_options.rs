use super::{OutboxMaxAttempts, OutboxRetryDelay};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OutboxRetryOptions {
    pub backoff: OutboxRetryDelay,
    pub max_attempts: OutboxMaxAttempts,
}
