use crate::unit_of_work::UnitOfWork;

use super::{Outbox, OutboxRelayError, OutboxRelayRunReport};

#[allow(async_fn_in_trait)]
pub trait OutboxRelay: Send + Sync {
    type Uow: UnitOfWork;
    type Outbox: Outbox;

    fn is_stop_requested(&self) -> bool;

    fn request_graceful_stop(&mut self);

    async fn run_forever(&self, uow: &mut Self::Uow) -> Result<(), OutboxRelayError>;

    async fn run_once(&self, uow: &mut Self::Uow) -> Result<OutboxRelayRunReport, OutboxRelayError>;
}
