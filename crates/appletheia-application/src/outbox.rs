pub mod outbox_id;
pub mod outbox_id_error;
pub mod outbox_relay;
pub mod outbox_relay_config;
pub mod outbox_relay_config_access;
pub mod outbox_relay_error;

pub use outbox_id::OutboxId;
pub use outbox_id_error::OutboxIdError;
pub use outbox_relay::OutboxRelay;
pub use outbox_relay_config::OutboxRelayConfig;
pub use outbox_relay_config_access::OutboxRelayConfigAccess;
pub use outbox_relay_error::OutboxRelayError;

use appletheia_domain::{AggregateVersion, EventId, OccurredAt};
use chrono::{DateTime, Utc};

use crate::event::{AggregateTypeOwned, EventSequence};
use crate::request_context::{CorrelationId, MessageId, RequestContext};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Outbox {
    pub id: OutboxId,
    pub event_sequence: EventSequence,
    pub event_id: EventId,
    pub aggregate_type: AggregateTypeOwned,
    pub aggregate_id: String,
    pub aggregate_version: AggregateVersion,
    pub payload: serde_json::Value,
    pub occurred_at: OccurredAt,
    pub correlation_id: CorrelationId,
    pub causation_id: MessageId,
    pub context: RequestContext,
    pub ordering_key: String,
    pub published_at: Option<DateTime<Utc>>,
    pub attempt_count: i64,
    pub next_attempt_after: DateTime<Utc>,
    pub lease_owner: Option<String>,
    pub lease_until: Option<DateTime<Utc>>,
}
