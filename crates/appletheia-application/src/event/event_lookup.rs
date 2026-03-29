use appletheia_domain::EventId;

use crate::request_context::{CausationId, CorrelationId};
use crate::unit_of_work::UnitOfWork;

use super::{EventEnvelope, EventLookupError, EventSequence};

#[allow(async_fn_in_trait)]
pub trait EventLookup: Send + Sync {
    type Uow: UnitOfWork;

    async fn max_event_sequence_by_causation_id(
        &self,
        uow: &mut Self::Uow,
        causation_id: CausationId,
    ) -> Result<Option<EventSequence>, EventLookupError>;

    async fn last_event_id_by_causation_id(
        &self,
        uow: &mut Self::Uow,
        causation_id: CausationId,
    ) -> Result<Option<EventId>, EventLookupError>;

    async fn events_by_causation_id(
        &self,
        uow: &mut Self::Uow,
        causation_id: CausationId,
    ) -> Result<Vec<EventEnvelope>, EventLookupError>;

    async fn events_by_correlation_id(
        &self,
        uow: &mut Self::Uow,
        correlation_id: CorrelationId,
    ) -> Result<Vec<EventEnvelope>, EventLookupError>;
}
