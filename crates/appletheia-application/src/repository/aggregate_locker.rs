use super::{AggregateLockMode, AggregateLockerError};
use crate::{aggregate::AggregateRef, unit_of_work::UnitOfWork};

/// Holds aggregate locks until the UnitOfWork ends. Shared locks cannot be upgraded.
#[allow(async_fn_in_trait)]
pub trait AggregateLocker: Send + Sync {
    type Uow: UnitOfWork;

    async fn lock(
        &self,
        uow: &mut Self::Uow,
        aggregate: &AggregateRef,
        lock_mode: AggregateLockMode,
    ) -> Result<(), AggregateLockerError>;

    /// Holds an exclusive lock, creating the lock row when absent.
    async fn create_if_not_exists(
        &self,
        uow: &mut Self::Uow,
        aggregate: &AggregateRef,
    ) -> Result<(), AggregateLockerError>;
}
