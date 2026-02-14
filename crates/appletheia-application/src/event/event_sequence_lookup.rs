use crate::event::EventSequence;
use crate::request_context::CausationId;
use crate::unit_of_work::UnitOfWork;

use super::EventSequenceLookupError;

#[allow(async_fn_in_trait)]
pub trait EventSequenceLookup: Send + Sync {
    type Uow: UnitOfWork;

    async fn max_event_sequence_by_causation_id(
        &self,
        uow: &mut Self::Uow,
        causation_id: CausationId,
    ) -> Result<Option<EventSequence>, EventSequenceLookupError>;
}
