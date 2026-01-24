pub mod snapshot_interval;
pub mod snapshot_policy;
pub mod unit_of_work_error;

pub use snapshot_interval::SnapshotInterval;
pub use snapshot_policy::SnapshotPolicy;
pub use unit_of_work_error::UnitOfWorkError;

use core::future::Future;
use std::error::Error;

#[allow(async_fn_in_trait)]
pub trait UnitOfWork: Send {
    async fn begin(&mut self) -> Result<(), UnitOfWorkError>;

    async fn commit(&mut self) -> Result<(), UnitOfWorkError>;

    async fn rollback(&mut self) -> Result<(), UnitOfWorkError>;

    fn is_in_transaction(&self) -> bool;

    async fn rollback_with_operation_error<E>(
        &mut self,
        operation_error: E,
    ) -> Result<E, UnitOfWorkError>
    where
        E: Error + Send + Sync + 'static,
    {
        match self.rollback().await {
            Ok(()) => Ok(operation_error),
            Err(rollback_error) => Err(UnitOfWorkError::OperationAndRollbackFailed {
                operation_error: Box::new(operation_error),
                rollback_error: Box::new(rollback_error),
            }),
        }
    }

    async fn run_in_transaction<
        F: FnOnce(&mut Self) -> Fut + Send,
        Fut: Future<Output = Result<T, E>> + Send,
        T: Send,
        E: Error + From<UnitOfWorkError> + Send + Sync + 'static,
    >(
        &mut self,
        operation: F,
    ) -> Result<T, E> {
        self.begin().await?;
        let result = operation(self).await;
        match result {
            Ok(value) => {
                self.commit().await?;
                Ok(value)
            }
            Err(error) => Err(self.rollback_with_operation_error(error).await?),
        }
    }
}
