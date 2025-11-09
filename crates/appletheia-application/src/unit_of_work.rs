pub mod unit_of_work_config;
pub mod unit_of_work_error;

pub use unit_of_work_config::UnitOfWorkConfig;
pub use unit_of_work_error::UnitOfWorkError;

use core::future::Future;
use std::error::Error;

use appletheia_domain::Aggregate;

#[allow(async_fn_in_trait)]
pub trait UnitOfWork<A: Aggregate> {
    fn config(&self) -> &UnitOfWorkConfig;

    async fn begin(&self) -> Result<(), UnitOfWorkError>;

    async fn commit(&self) -> Result<(), UnitOfWorkError>;

    async fn rollback(&self) -> Result<(), UnitOfWorkError>;

    fn is_active(&self) -> bool;

    async fn save(&mut self, aggregate: &mut A) -> Result<(), UnitOfWorkError>;

    async fn run_in_transaction<
        F: FnOnce(&mut Self) -> Fut + Send,
        Fut: Future<Output = Result<T, E>> + Send,
        T: Send,
        E: Error + From<UnitOfWorkError> + Send + Sync + 'static,
    >(
        &mut self,
        operation: F,
    ) -> Result<T, E> {
        if !self.is_active() {
            self.begin().await?;
            let result = operation(self).await;
            match result {
                Ok(value) => {
                    self.commit().await?;
                    Ok(value)
                }
                Err(error) => match self.rollback().await {
                    Ok(()) => Err(error),
                    Err(rollback_error) => Err(UnitOfWorkError::OperationAndRollbackFailed {
                        operation_error: Box::new(error),
                        rollback_error: Box::new(rollback_error),
                    }
                    .into()),
                },
            }
        } else {
            operation(self).await
        }
    }
}
