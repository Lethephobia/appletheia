use serde::{Serialize, de::DeserializeOwned};

use crate::request_context::CorrelationId;
use crate::unit_of_work::UnitOfWork;

use super::{SagaNameOwned, SagaRun, SagaRunStoreError};

#[allow(async_fn_in_trait)]
pub trait SagaRunStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn read<C: Serialize + DeserializeOwned + Send + Sync + 'static>(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaNameOwned,
        correlation_id: CorrelationId,
    ) -> Result<Option<SagaRun<C>>, SagaRunStoreError>;

    async fn exists(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaNameOwned,
        correlation_id: CorrelationId,
    ) -> Result<bool, SagaRunStoreError>;

    async fn write<C: Serialize + DeserializeOwned + Send + Sync + 'static>(
        &self,
        uow: &mut Self::Uow,
        run: &SagaRun<C>,
    ) -> Result<(), SagaRunStoreError>;
}
