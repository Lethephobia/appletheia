pub mod snapshot_interval;
pub mod snapshot_policy;
pub mod unit_of_work_error;
pub mod unit_of_work_factory;
pub mod unit_of_work_factory_error;

pub use snapshot_interval::SnapshotInterval;
pub use snapshot_policy::SnapshotPolicy;
pub use unit_of_work_error::UnitOfWorkError;
pub use unit_of_work_factory::UnitOfWorkFactory;
pub use unit_of_work_factory_error::UnitOfWorkFactoryError;

use std::error::Error;

#[allow(async_fn_in_trait)]
pub trait UnitOfWork: Send {
    async fn commit(self) -> Result<(), UnitOfWorkError>
    where
        Self: Sized;

    async fn rollback(self) -> Result<(), UnitOfWorkError>
    where
        Self: Sized;

    async fn rollback_with_operation_error<E>(
        self,
        operation_error: E,
    ) -> Result<E, UnitOfWorkError>
    where
        E: Error + Send + Sync + 'static,
        Self: Sized,
    {
        match self.rollback().await {
            Ok(()) => Ok(operation_error),
            Err(rollback_error) => Err(UnitOfWorkError::OperationAndRollbackFailed {
                operation_error: Box::new(operation_error),
                rollback_error: Box::new(rollback_error),
            }),
        }
    }
}
