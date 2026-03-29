use appletheia_domain::EventId;

use crate::unit_of_work::UnitOfWork;

use super::{ProjectorNameOwned, ProjectorProcessedEventStoreError};

#[allow(async_fn_in_trait)]
pub trait ProjectorProcessedEventStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn are_all_processed(
        &self,
        uow: &mut Self::Uow,
        projector_name: ProjectorNameOwned,
        event_ids: &[EventId],
    ) -> Result<bool, ProjectorProcessedEventStoreError>;

    async fn is_processed(
        &self,
        uow: &mut Self::Uow,
        projector_name: ProjectorNameOwned,
        event_id: EventId,
    ) -> Result<bool, ProjectorProcessedEventStoreError>;

    async fn mark_processed(
        &self,
        uow: &mut Self::Uow,
        projector_name: ProjectorNameOwned,
        event_id: EventId,
    ) -> Result<bool, ProjectorProcessedEventStoreError>;

    async fn reset(
        &self,
        uow: &mut Self::Uow,
        projector_name: ProjectorNameOwned,
    ) -> Result<(), ProjectorProcessedEventStoreError>;
}
