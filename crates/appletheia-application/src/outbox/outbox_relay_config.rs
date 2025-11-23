use super::{OutboxBatchSize, OutboxLeaseDuration, OutboxRelayInstance};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutboxRelayConfig {
    pub instance: OutboxRelayInstance,
    pub batch_size: OutboxBatchSize,
    pub lease_duration: OutboxLeaseDuration,
}
