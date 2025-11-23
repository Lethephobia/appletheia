pub mod ordering_key;
pub mod ordering_key_error;
pub mod outbox_id;
pub mod outbox_id_error;
pub mod outbox_published_at;
pub mod outbox_relay;
pub mod outbox_relay_config;
pub mod outbox_relay_config_access;
pub mod outbox_relay_error;

pub use ordering_key::OrderingKey;
pub use ordering_key_error::OrderingKeyError;
pub use outbox_id::OutboxId;
pub use outbox_id_error::OutboxIdError;
pub use outbox_published_at::OutboxPublishedAt;
pub use outbox_relay::OutboxRelay;
pub use outbox_relay_config::OutboxRelayConfig;
pub use outbox_relay_config_access::OutboxRelayConfigAccess;
pub use outbox_relay_error::OutboxRelayError;

use appletheia_domain::{AggregateVersion, EventId, OccurredAt};
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
    pub occurred_at: OccurredAt,
    pub correlation_id: CorrelationId,
    pub causation_id: MessageId,
    pub context: RequestContext,
    pub published_at: Option<OutboxPublishedAt>,
    pub attempt_count: i64,
    pub next_attempt_after: DateTime<Utc>,
    pub lease_owner: Option<String>,
    pub lease_until: Option<DateTime<Utc>>,
}

impl Outbox {
    pub fn ordering_key(&self) -> OrderingKey {
        OrderingKey::new(self.aggregate_type.clone(), self.aggregate_id)
    }
}
