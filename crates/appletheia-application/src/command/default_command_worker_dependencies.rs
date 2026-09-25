use crate::messaging::Subscriber;
use crate::outbox::command_failure::CommandFailureOutboxEnqueuer;
use crate::unit_of_work::UnitOfWorkFactory;

use super::{CommandDispatcher, CommandEnvelope, CommandExecutionStore, CommandSelector};

/// Collects dependencies used by the default asynchronous command worker.
pub struct DefaultCommandWorkerDependencies<D, S, ES, FE, U>
where
    D: CommandDispatcher,
    S: Subscriber<CommandEnvelope, Selector = CommandSelector>,
    ES: CommandExecutionStore<Uow = D::Uow>,
    FE: CommandFailureOutboxEnqueuer<Uow = D::Uow>,
    U: UnitOfWorkFactory<Uow = D::Uow>,
{
    pub dispatcher: D,
    pub subscriber: S,
    pub execution_store: ES,
    pub failure_outbox_enqueuer: FE,
    pub uow_factory: U,
}
