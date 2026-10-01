use appletheia_domain::EventId;

use crate::request_context::MessageId;
use crate::unit_of_work::UnitOfWork;

use super::{SagaInstance, SagaInstanceStoreError, SagaNameOwned, SagaState, SagaStep};

#[allow(async_fn_in_trait)]
pub trait SagaInstanceStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn find_by_start_event_id<S: SagaState, T: SagaStep>(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaNameOwned,
        start_event_id: EventId,
    ) -> Result<Option<SagaInstance<S, T>>, SagaInstanceStoreError>;

    async fn find_by_dispatched_command_message_id<S: SagaState, T: SagaStep>(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaNameOwned,
        dispatched_command_message_id: MessageId,
    ) -> Result<Option<SagaInstance<S, T>>, SagaInstanceStoreError>;

    /// Persists this instance and its dispatched commands within the unit of work.
    ///
    /// A different instance with the same saga name and start event ID must cause
    /// a conflict, rather than overwrite the existing instance's state.
    async fn save<S: SagaState, T: SagaStep>(
        &self,
        uow: &mut Self::Uow,
        instance: &SagaInstance<S, T>,
    ) -> Result<(), SagaInstanceStoreError>;
}
