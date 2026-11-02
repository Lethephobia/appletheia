use super::{
    ReadModelWatchMaxAttempts, ReadModelWatchRetryBackoffMultiplier, ReadModelWatchRetryDelay,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ReadModelWatchRetryOptions {
    pub max_attempts: ReadModelWatchMaxAttempts,
    pub base_delay: ReadModelWatchRetryDelay,
    pub max_delay: ReadModelWatchRetryDelay,
    pub multiplier: ReadModelWatchRetryBackoffMultiplier,
}

impl Default for ReadModelWatchRetryOptions {
    fn default() -> Self {
        Self {
            max_attempts: Default::default(),
            base_delay: Default::default(),
            max_delay: ReadModelWatchRetryDelay::default_max(),
            multiplier: Default::default(),
        }
    }
}
