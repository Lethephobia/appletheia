use crate::unit_of_work::UnitOfWork;

use super::{Projector, ProjectorRebuildReport, ProjectorRebuilderError};

#[allow(async_fn_in_trait)]
pub trait ProjectorRebuilder: Send {
    type Uow: UnitOfWork;

    fn is_stop_requested(&self) -> bool;

    fn request_graceful_stop(&mut self);

    /// Replays a newly introduced projector through the current event-feed position.
    ///
    /// The projector must use a fresh name so its checkpoint and processed-event state start
    /// empty. Snapshot serving and the live worker must remain stopped until catch-up completes.
    /// Changes recorded during catch-up are not published; clients start from a snapshot and
    /// observe only later live changes.
    async fn run_until_idle<PJ: Projector<Uow = Self::Uow>>(
        &mut self,
        projector: &PJ,
    ) -> Result<ProjectorRebuildReport, ProjectorRebuilderError>;
}
