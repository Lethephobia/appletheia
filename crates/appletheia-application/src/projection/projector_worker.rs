use super::{Projector, ProjectorWorkerError};
use crate::unit_of_work::UnitOfWork;

#[allow(async_fn_in_trait)]
pub trait ProjectorWorker: Send + Sync {
    type Uow: UnitOfWork;

    fn is_stop_requested(&self) -> bool;

    fn request_graceful_stop(&self);

    async fn run_forever<PJ>(&self, projector: &PJ) -> Result<(), ProjectorWorkerError>
    where
        PJ: Projector<Uow = Self::Uow>;
}
