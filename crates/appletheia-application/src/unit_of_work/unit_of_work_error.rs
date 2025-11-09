use std::error::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UnitOfWorkError {
    #[error("begin failed {0}")]
    BeginFailed(#[source] Box<dyn Error + Send + Sync + 'static>),

    #[error("commit failed {0}")]
    CommitFailed(#[source] Box<dyn Error + Send + Sync + 'static>),

    #[error("rollback failed {0}")]
    RollbackFailed(#[source] Box<dyn Error + Send + Sync + 'static>),

    #[error("operation and rollback failed {operation_error} {rollback_error}")]
    OperationAndRollbackFailed {
        #[source]
        operation_error: Box<dyn Error + Send + Sync + 'static>,
        rollback_error: Box<dyn Error + Send + Sync + 'static>,
    },
}
