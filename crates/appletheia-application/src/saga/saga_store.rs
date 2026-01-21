use crate::request_context::CorrelationId;
use crate::unit_of_work::UnitOfWork;

use super::{SagaInstance, SagaName, SagaState, SagaStoreError};

#[allow(async_fn_in_trait)]
pub trait SagaStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn load<S: SagaState>(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaName,
        correlation_id: CorrelationId,
    ) -> Result<SagaInstance<S>, SagaStoreError>;

    async fn save<S: SagaState>(
        &self,
        uow: &mut Self::Uow,
        instance: &SagaInstance<S>,
    ) -> Result<(), SagaStoreError>;
}
