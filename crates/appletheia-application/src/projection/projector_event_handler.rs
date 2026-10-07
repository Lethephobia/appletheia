use std::{error::Error, future::Future, pin::Pin};

use crate::{event::EventEnvelope, unit_of_work::UnitOfWork};

/// Erases the event type while borrowing the handler and transaction for execution.
pub(crate) trait ProjectorEventHandler<U: UnitOfWork, E: Error + Send + Sync + 'static>:
    Send + Sync
{
    fn handle<'a>(
        &'a self,
        uow: &'a mut U,
        event: &'a EventEnvelope,
    ) -> Pin<Box<dyn Future<Output = Result<(), E>> + 'a>>;
}
