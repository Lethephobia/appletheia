pub mod snapshot_interval;
pub mod snapshot_policy;
pub mod unit_of_work_config;
pub mod unit_of_work_error;

pub use snapshot_interval::SnapshotInterval;
pub use snapshot_policy::SnapshotPolicy;
pub use unit_of_work_config::UnitOfWorkConfig;
pub use unit_of_work_error::UnitOfWorkError;

use core::future::Future;
use std::error::Error;

use appletheia_domain::{Aggregate, AggregateVersion, Event, Snapshot};

#[allow(async_fn_in_trait)]
pub trait UnitOfWork<A: Aggregate> {
    fn config(&self) -> &UnitOfWorkConfig;

    async fn begin(&mut self) -> Result<(), UnitOfWorkError<A>>;

    async fn commit(&mut self) -> Result<(), UnitOfWorkError<A>>;

    async fn rollback(&mut self) -> Result<(), UnitOfWorkError<A>>;

    fn is_active(&self) -> bool;

    async fn write_events(
        &mut self,
        events: &[Event<A::Id, A::EventPayload>],
    ) -> Result<(), UnitOfWorkError<A>>;

    async fn write_outbox(
        &mut self,
        events: &[Event<A::Id, A::EventPayload>],
    ) -> Result<(), UnitOfWorkError<A>>;

    async fn write_snapshot(
        &mut self,
        snapshot: &Snapshot<A::State>,
    ) -> Result<(), UnitOfWorkError<A>>;

    async fn read_latest_snapshot_version(
        &self,
        aggregate_id: A::Id,
    ) -> Result<Option<AggregateVersion>, UnitOfWorkError<A>>;

    async fn save(&mut self, aggregate: &mut A) -> Result<(), UnitOfWorkError<A>> {
        let events = aggregate.uncommitted_events();
        self.write_events(events).await?;
        self.write_outbox(events).await?;
        match self.config().snapshot_policy {
            SnapshotPolicy::Disabled => {}
            SnapshotPolicy::AtLeast { minimum_interval } => {
                let aggregate_id = aggregate
                    .aggregate_id()
                    .ok_or(UnitOfWorkError::<A>::AggregateNoState)?;
                let current_version = aggregate.version().as_u64();
                let latest_snapshot_version = self
                    .read_latest_snapshot_version(aggregate_id)
                    .await?
                    .map(|v| v.as_u64())
                    .unwrap_or(0);
                if current_version.saturating_sub(latest_snapshot_version)
                    >= minimum_interval.as_u64()
                {
                    let snapshot = aggregate
                        .to_snapshot()
                        .map_err(UnitOfWorkError::<A>::Aggregate)?;
                    self.write_snapshot(&snapshot).await?;
                }
            }
        }
        aggregate.clear_uncommitted_events();
        Ok(())
    }

    async fn run_in_transaction<
        F: FnOnce(&mut Self) -> Fut + Send,
        Fut: Future<Output = Result<T, E>> + Send,
        T: Send,
        E: Error + From<UnitOfWorkError<A>> + Send + Sync + 'static,
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
                    Err(rollback_error) => Err(UnitOfWorkError::<A>::OperationAndRollbackFailed {
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
