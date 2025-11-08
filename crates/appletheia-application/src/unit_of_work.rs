pub mod unit_of_work_error;

pub use unit_of_work_error::UnitOfWorkError;

use std::error::Error;

use appletheia_domain::Aggregate;

#[allow(async_fn_in_trait)]
pub trait UnitOfWork<A: Aggregate> {
    async fn begin(&self) -> Result<(), UnitOfWorkError>;

    async fn commit(&self) -> Result<(), UnitOfWorkError>;

    async fn rollback(&self) -> Result<(), UnitOfWorkError>;

    async fn is_active(&self) -> bool;

    async fn run_in_transaction<T, E: Error + From<UnitOfWorkError> + Send + Sync + 'static>(
        &self,
        operation: impl FnOnce(&mut A) -> Result<T, E>,
    ) -> Result<T, E>;
}
