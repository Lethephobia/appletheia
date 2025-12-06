use super::{
    OutboxBatchSize, OutboxLeaseDuration, OutboxPollingOptions, OutboxRelayInstance,
    OutboxRetryOptions,
};

#[derive(Clone, Debug, PartialEq)]
pub struct OutboxRelayConfig {
    pub instance: OutboxRelayInstance,
    pub batch_size: OutboxBatchSize,
    pub lease_duration: OutboxLeaseDuration,
    pub retry_options: OutboxRetryOptions,
    pub polling_options: OutboxPollingOptions,
}
