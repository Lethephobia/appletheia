use serde::{Serialize, de::DeserializeOwned};

use appletheia_domain::EventId;

use crate::request_context::MessageId;
use crate::unit_of_work::UnitOfWork;

use super::{SagaNameOwned, SagaRun, SagaRunStoreError};

#[allow(async_fn_in_trait)]
pub trait SagaRunStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn read_by_trigger_event<C: Serialize + DeserializeOwned + Send + Sync + 'static>(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaNameOwned,
        trigger_event_id: EventId,
    ) -> Result<Option<SagaRun<C>>, SagaRunStoreError>;

    async fn read_by_dispatched_command_message<
        C: Serialize + DeserializeOwned + Send + Sync + 'static,
    >(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaNameOwned,
        dispatched_command_message_id: MessageId,
    ) -> Result<Option<SagaRun<C>>, SagaRunStoreError>;

    async fn write<C: Serialize + DeserializeOwned + Send + Sync + 'static>(
        &self,
        uow: &mut Self::Uow,
        run: &SagaRun<C>,
    ) -> Result<(), SagaRunStoreError>;
}
