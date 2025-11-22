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

use appletheia_domain::{
    Aggregate, Event, Repository, Snapshot, SnapshotReader, TrySnapshotReaderProvider,
};

#[allow(async_fn_in_trait)]
pub trait UnitOfWork<A: Aggregate>:
    TrySnapshotReaderProvider<A, Error = UnitOfWorkError<A>>
{
    type Repository<'c>: Repository<A>
    where
        Self: 'c;

    fn config(&self) -> &UnitOfWorkConfig;

    fn repository(&mut self) -> Result<Self::Repository<'_>, UnitOfWorkError<A>>;

    async fn begin(&mut self) -> Result<(), UnitOfWorkError<A>>;

    async fn commit(&mut self) -> Result<(), UnitOfWorkError<A>>;

    async fn rollback(&mut self) -> Result<(), UnitOfWorkError<A>>;

    fn is_in_transaction(&self) -> bool;

    async fn write_events_and_outbox(
        &mut self,
        events: &[Event<A::Id, A::EventPayload>],
    ) -> Result<(), UnitOfWorkError<A>>;

    async fn write_snapshot(
        &mut self,
        snapshot: &Snapshot<A::State>,
    ) -> Result<(), UnitOfWorkError<A>>;

    async fn save(&mut self, aggregate: &mut A) -> Result<(), UnitOfWorkError<A>> {
        let events = aggregate.uncommitted_events();
        self.write_events_and_outbox(events).await?;
        match self.config().snapshot_policy {
            SnapshotPolicy::Disabled => {}
            SnapshotPolicy::AtLeast { minimum_interval } => {
                let aggregate_id = aggregate
                    .aggregate_id()
                    .ok_or(UnitOfWorkError::<A>::AggregateNoState)?;
                let current_version = aggregate.version().as_u64();
                let latest_snapshot_version = {
                    let mut reader = self.try_snapshot_reader()?;
                    reader
                        .read_latest_snapshot(aggregate_id, None)
                        .await
                        .map_err(UnitOfWorkError::<A>::SnapshotReader)?
                        .as_ref()
                        .map(|snapshot| snapshot.aggregate_version().as_u64())
                        .unwrap_or(0)
                };
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
        if !self.is_in_transaction() {
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
