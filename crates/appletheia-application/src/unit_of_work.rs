pub mod unit_of_work_error;
pub mod unit_of_work_factory;
pub mod unit_of_work_factory_error;

pub use unit_of_work_error::UnitOfWorkError;
pub use unit_of_work_factory::UnitOfWorkFactory;
pub use unit_of_work_factory_error::UnitOfWorkFactoryError;

use std::error::Error;

/// Represents a transactional application work scope.
///
/// A `UnitOfWork` is typically created for a single command, saga step, or
/// projection update and is responsible for either committing or rolling back
/// all changes performed within that scope.
#[allow(async_fn_in_trait)]
pub trait UnitOfWork: Send {
    /// Commits all changes performed within this unit of work.
    async fn commit(self) -> Result<(), UnitOfWorkError>
    where
        Self: Sized;

    /// Rolls back all changes performed within this unit of work.
    async fn rollback(self) -> Result<(), UnitOfWorkError>
    where
        Self: Sized;

    /// Rolls back the unit of work and returns the original operation error.
    ///
    /// If the rollback itself fails, both errors are combined into
    /// `UnitOfWorkError::OperationAndRollbackFailed`.
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
