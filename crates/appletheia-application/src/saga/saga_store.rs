use appletheia_domain::EventId;

use crate::request_context::CorrelationId;
use crate::unit_of_work::UnitOfWork;

use super::{SagaInstanceRow, SagaInstanceUpdate, SagaName, SagaStoreError};

#[allow(async_fn_in_trait)]
pub trait SagaStore {
    type Uow: UnitOfWork;

    async fn load_for_update(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaName,
        correlation_id: CorrelationId,
    ) -> Result<Option<SagaInstanceRow>, SagaStoreError>;

    async fn insert_instance_if_absent(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaName,
        correlation_id: CorrelationId,
        initial_state: serde_json::Value,
    ) -> Result<(), SagaStoreError>;

    async fn update_instance(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaName,
        correlation_id: CorrelationId,
        update: SagaInstanceUpdate,
    ) -> Result<(), SagaStoreError>;

    async fn mark_event_processed(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaName,
        correlation_id: CorrelationId,
        event_id: EventId,
    ) -> Result<bool, SagaStoreError>;
}
