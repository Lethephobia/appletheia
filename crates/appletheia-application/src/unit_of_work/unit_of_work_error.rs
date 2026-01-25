use thiserror::Error;

#[derive(Debug, Error)]
pub enum UnitOfWorkError {
    #[error("commit failed {0}")]
    CommitFailed(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("rollback failed {0}")]
    RollbackFailed(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("operation and rollback failed {operation_error} {rollback_error}")]
    OperationAndRollbackFailed {
        #[source]
        operation_error: Box<dyn std::error::Error + Send + Sync + 'static>,
        rollback_error: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
}
