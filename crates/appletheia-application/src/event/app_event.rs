use serde::{Deserialize, Serialize};

use appletheia_domain::{AggregateType, AggregateVersion, EventId, EventOccurredAt};

use crate::event::{AggregateIdOwned, EventPayloadOwned, EventSequence};
use crate::request_context::{CausationId, CorrelationId, RequestContext};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct AppEvent<AT: AggregateType> {
    pub event_sequence: EventSequence,
    pub event_id: EventId,
    pub aggregate_type: AT,
    pub aggregate_id: AggregateIdOwned,
    pub aggregate_version: AggregateVersion,
    pub payload: EventPayloadOwned,
    pub occurred_at: EventOccurredAt,
    pub correlation_id: CorrelationId,
    pub causation_id: CausationId,
    pub context: RequestContext,
}
