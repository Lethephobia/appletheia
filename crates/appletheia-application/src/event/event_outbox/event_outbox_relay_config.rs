use super::{
    EventOutboxBatchSize, EventOutboxLeaseDuration, EventOutboxPollingOptions,
    EventOutboxRelayInstance, EventOutboxRetryOptions,
};

#[derive(Clone, Debug, PartialEq)]
pub struct EventOutboxRelayConfig {
    pub instance: EventOutboxRelayInstance,
    pub batch_size: EventOutboxBatchSize,
    pub lease_duration: EventOutboxLeaseDuration,
    pub retry_options: EventOutboxRetryOptions,
    pub polling_options: EventOutboxPollingOptions,
}
