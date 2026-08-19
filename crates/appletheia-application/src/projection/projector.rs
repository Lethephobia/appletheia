use std::error::Error;

use crate::event::EventEnvelope;
use crate::read_model::{
    MaterializationEventContext, ReadModelFragment, ReadModelFragmentPartition,
};
use crate::unit_of_work::UnitOfWork;

use super::ProjectorSpec;

/// Projects events into one physical read-model fragment type.
#[allow(async_fn_in_trait)]
pub trait Projector: Send + Sync {
    type Spec: ProjectorSpec;
    /// Identifies the only physical fragment this projector may change.
    type Fragment: ReadModelFragment;
    type Uow: UnitOfWork;
    type Error: Error + Send + Sync + 'static;

    /// Materializes one event and returns the physical partitions it invalidated.
    async fn project(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        event: &EventEnvelope,
    ) -> Result<Vec<ReadModelFragmentPartition<Self::Fragment>>, Self::Error>;
}
