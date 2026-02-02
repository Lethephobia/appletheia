use crate::event::EventSequence;
use crate::unit_of_work::UnitOfWork;

use super::{ProjectionCheckpointStoreError, ProjectorNameOwned};

#[allow(async_fn_in_trait)]
pub trait ProjectionCheckpointStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn load(
        &self,
        uow: &mut Self::Uow,
        projector_name: ProjectorNameOwned,
    ) -> Result<Option<EventSequence>, ProjectionCheckpointStoreError>;

    async fn save(
        &self,
        uow: &mut Self::Uow,
        projector_name: ProjectorNameOwned,
        event_sequence: EventSequence,
    ) -> Result<(), ProjectionCheckpointStoreError>;

    async fn reset(
        &self,
        uow: &mut Self::Uow,
        projector_name: ProjectorNameOwned,
    ) -> Result<(), ProjectionCheckpointStoreError>;
}
