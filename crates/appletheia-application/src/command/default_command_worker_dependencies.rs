/// Collects dependencies used by the default asynchronous command worker.
pub struct DefaultCommandWorkerDependencies<D, S, ES, FE, U> {
    pub dispatcher: D,
    pub subscriber: S,
    pub execution_store: ES,
    pub failure_outbox_enqueuer: FE,
    pub uow_factory: U,
}
