use serde::{Deserialize, Serialize};

use appletheia_domain::{AggregateVersion, EventId, EventOccurredAt};

use crate::event::{AggregateIdValue, AggregateTypeOwned, EventSequence, SerializedEventPayload};
use crate::request_context::{CausationId, CorrelationId, RequestContext};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub event_sequence: EventSequence,
    pub event_id: EventId,
    pub aggregate_type: AggregateTypeOwned,
    pub aggregate_id: AggregateIdValue,
    pub aggregate_version: AggregateVersion,
    pub payload: SerializedEventPayload,
    pub occurred_at: EventOccurredAt,
    pub correlation_id: CorrelationId,
    pub causation_id: CausationId,
    pub context: RequestContext,
}
