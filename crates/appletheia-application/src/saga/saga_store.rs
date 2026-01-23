use crate::request_context::CorrelationId;
use crate::unit_of_work::UnitOfWork;

use super::{SagaInstance, SagaName, SagaState, SagaStoreError};

#[allow(async_fn_in_trait)]
pub trait SagaStore: Send + Sync {
    type Uow: UnitOfWork;
    type SagaName: SagaName;

    async fn load<S: SagaState>(
        &self,
        uow: &mut Self::Uow,
        saga_name: Self::SagaName,
        correlation_id: CorrelationId,
    ) -> Result<SagaInstance<Self::SagaName, S>, SagaStoreError>;

    async fn save<S: SagaState>(
        &self,
        uow: &mut Self::Uow,
        instance: &SagaInstance<Self::SagaName, S>,
    ) -> Result<(), SagaStoreError>;
}
