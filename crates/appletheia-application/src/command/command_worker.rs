use super::{CommandHandler, CommandWorkerError};
use crate::unit_of_work::UnitOfWork;

#[allow(async_fn_in_trait)]
pub trait CommandWorker: Send + Sync {
    type Uow: UnitOfWork;

    fn is_stop_requested(&self) -> bool;

    fn request_graceful_stop(&self);

    async fn run_forever<H>(&self, handler: &H) -> Result<(), CommandWorkerError>
    where
        H: CommandHandler<Uow = Self::Uow>;
}
