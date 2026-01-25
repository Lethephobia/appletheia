use std::error::Error;

use super::{UnitOfWork, UnitOfWorkError, UnitOfWorkFactoryError};

#[allow(async_fn_in_trait)]
pub trait UnitOfWorkFactory: Send + Sync {
    type Uow: UnitOfWork;

    async fn begin(&self) -> Result<Self::Uow, UnitOfWorkFactoryError>;

    async fn run_in_transaction<
        F: FnOnce(&mut Self::Uow) -> Fut + Send,
        Fut: Future<Output = Result<T, E>> + Send,
        T: Send,
        E: Error + From<UnitOfWorkError> + From<UnitOfWorkFactoryError> + Send + Sync + 'static,
    >(
        &self,
        operation: F,
    ) -> Result<T, E> {
        let mut uow = self.begin().await?;
        let result = operation(&mut uow).await;
        match result {
            Ok(value) => {
                uow.commit().await?;
                Ok(value)
            }
            Err(error) => Err(uow.rollback_with_operation_error(error).await?),
        }
    }
}
