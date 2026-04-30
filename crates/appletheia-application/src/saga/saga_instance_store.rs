use crate::request_context::{CorrelationId, MessageId};
use crate::unit_of_work::UnitOfWork;

use super::{SagaInstance, SagaInstanceStoreError, SagaNameOwned, SagaState};

#[allow(async_fn_in_trait)]
pub trait SagaInstanceStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn find_by_correlation_id<S: SagaState>(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaNameOwned,
        correlation_id: CorrelationId,
    ) -> Result<Option<SagaInstance<S>>, SagaInstanceStoreError>;

    async fn find_by_dispatched_command_message_id<S: SagaState>(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaNameOwned,
        dispatched_command_message_id: MessageId,
    ) -> Result<Option<SagaInstance<S>>, SagaInstanceStoreError>;

    async fn save<S: SagaState>(
        &self,
        uow: &mut Self::Uow,
        instance: &SagaInstance<S>,
    ) -> Result<(), SagaInstanceStoreError>;
}
