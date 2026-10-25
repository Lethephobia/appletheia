use crate::unit_of_work::UnitOfWork;

use super::ReadModelWatchNotificationWorkerError;

#[allow(async_fn_in_trait)]
pub trait ReadModelWatchNotificationWorker: Send + Sync {
    type Uow: UnitOfWork;

    fn is_stop_requested(&self) -> bool;

    fn request_graceful_stop(&self);

    async fn run_forever(&self) -> Result<(), ReadModelWatchNotificationWorkerError>;
}
