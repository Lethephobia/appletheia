use crate::unit_of_work::UnitOfWork;

use super::{ProjectionCatchUpError, ProjectionCatchUpReport, Projector};

#[allow(async_fn_in_trait)]
pub trait ProjectionCatchUp: Send {
    type Uow: UnitOfWork;

    fn is_stop_requested(&self) -> bool;

    fn request_graceful_stop(&self);

    /// Applies matching events from the saved checkpoint until no events remain.
    ///
    /// Use this for startup catch-up or to build a fresh projection. This does not clear
    /// projection data, checkpoints, or processed-event markers. Keep read-model serving
    /// and the live worker stopped until catch-up completes.
    async fn run_until_idle<PJ: Projector<Uow = Self::Uow>>(
        &mut self,
        projector: &PJ,
    ) -> Result<ProjectionCatchUpReport, ProjectionCatchUpError>;
}
