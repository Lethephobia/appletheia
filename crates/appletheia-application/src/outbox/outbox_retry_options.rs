use super::{OutboxMaxAttempts, OutboxRetryBackoffMultiplier, OutboxRetryDelay};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OutboxRetryOptions {
    pub max_attempts: OutboxMaxAttempts,
    pub base_delay: OutboxRetryDelay,
    pub max_delay: OutboxRetryDelay,
    pub multiplier: OutboxRetryBackoffMultiplier,
}
