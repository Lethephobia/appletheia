use appletheia_domain::EventId;

use crate::request_context::CausationId;
use crate::unit_of_work::UnitOfWork;

use super::{EventEnvelope, EventLookupError};

#[allow(async_fn_in_trait)]
pub trait EventLookup: Send + Sync {
    type Uow: UnitOfWork;

    async fn events_by_causation_id(
        &self,
        uow: &mut Self::Uow,
        causation_id: CausationId,
    ) -> Result<Vec<EventEnvelope>, EventLookupError>;

    async fn events_by_event_ids(
        &self,
        uow: &mut Self::Uow,
        event_ids: &[EventId],
    ) -> Result<Vec<EventEnvelope>, EventLookupError>;
}
