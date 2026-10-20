use super::{OutboxBatchSize, OutboxLeaseDuration, OutboxPollingOptions, OutboxRetryOptions};

#[derive(Clone, Debug, PartialEq)]
pub struct OutboxRelayConfig {
    pub batch_size: OutboxBatchSize,
    pub lease_duration: OutboxLeaseDuration,
    pub retry_options: OutboxRetryOptions,
    pub polling_options: OutboxPollingOptions,
}
