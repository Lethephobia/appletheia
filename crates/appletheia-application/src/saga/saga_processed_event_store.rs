use appletheia_domain::EventId;

use crate::request_context::CorrelationId;
use crate::unit_of_work::UnitOfWork;

use super::{SagaName, SagaProcessedEventStoreError};

#[allow(async_fn_in_trait)]
pub trait SagaProcessedEventStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn mark_processed(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaName,
        correlation_id: CorrelationId,
        event_id: EventId,
    ) -> Result<bool, SagaProcessedEventStoreError>;
}
