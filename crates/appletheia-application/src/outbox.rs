pub mod ordering_key;
pub mod ordering_key_error;
pub mod outbox_attempt_count;
pub mod outbox_attempt_count_error;
pub mod outbox_id;
pub mod outbox_id_error;
pub mod outbox_next_attempt_at;
pub mod outbox_published_at;
pub mod outbox_relay;
pub mod outbox_relay_config;
pub mod outbox_relay_config_access;
pub mod outbox_relay_error;
pub mod outbox_relay_instance;
pub mod outbox_relay_instance_error;
pub mod outbox_relay_instance_id;
pub mod outbox_relay_process_id;

pub use ordering_key::OrderingKey;
pub use ordering_key_error::OrderingKeyError;
pub use outbox_attempt_count::OutboxAttemptCount;
pub use outbox_attempt_count_error::OutboxAttemptCountError;
pub use outbox_id::OutboxId;
pub use outbox_id_error::OutboxIdError;
pub use outbox_next_attempt_at::OutboxNextAttemptAt;
pub use outbox_published_at::OutboxPublishedAt;
pub use outbox_relay::OutboxRelay;
pub use outbox_relay_config::OutboxRelayConfig;
pub use outbox_relay_config_access::OutboxRelayConfigAccess;
pub use outbox_relay_error::OutboxRelayError;
pub use outbox_relay_instance::OutboxRelayInstance;
pub use outbox_relay_instance_error::OutboxRelayInstanceError;
pub use outbox_relay_instance_id::OutboxRelayInstanceId;
pub use outbox_relay_process_id::OutboxRelayProcessId;

use appletheia_domain::{AggregateVersion, EventId, EventOccurredAt};
use chrono::{DateTime, Utc};

use crate::event::{AggregateIdOwned, AggregateTypeOwned, EventPayloadOwned, EventSequence};
use crate::request_context::{CorrelationId, MessageId, RequestContext};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Outbox {
    pub id: OutboxId,
    pub event_sequence: EventSequence,
    pub event_id: EventId,
    pub aggregate_type: AggregateTypeOwned,
    pub aggregate_id: AggregateIdOwned,
    pub aggregate_version: AggregateVersion,
    pub payload: EventPayloadOwned,
    pub occurred_at: EventOccurredAt,
    pub correlation_id: CorrelationId,
    pub causation_id: MessageId,
    pub context: RequestContext,
    pub published_at: Option<OutboxPublishedAt>,
    pub attempt_count: OutboxAttemptCount,
    pub next_attempt_after: OutboxNextAttemptAt,
    pub lease_owner: Option<OutboxRelayInstance>,
    pub lease_until: Option<DateTime<Utc>>,
}

impl Outbox {
    pub fn ordering_key(&self) -> OrderingKey {
        OrderingKey::new(self.aggregate_type.clone(), self.aggregate_id)
    }
}
