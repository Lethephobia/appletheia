pub mod repository_config;
pub mod repository_config_access;
pub mod repository_error;

pub use repository_config::RepositoryConfig;
pub use repository_config_access::RepositoryConfigAccess;
pub use repository_error::RepositoryError;

use std::ops::Bound;

use appletheia_domain::{Aggregate, AggregateError, AggregateVersion, AggregateVersionRange};

use crate::event::{EventReader, EventReaderAccess, EventWriter, EventWriterAccess};
use crate::request_context::RequestContext;
use crate::snapshot::{SnapshotReader, SnapshotReaderAccess, SnapshotWriter, SnapshotWriterAccess};
use crate::unit_of_work::{SnapshotPolicy, UnitOfWork};

#[allow(async_fn_in_trait)]
pub trait Repository<A: Aggregate>:
    RepositoryConfigAccess
    + EventReaderAccess<A>
    + EventWriterAccess<A>
    + SnapshotReaderAccess<A>
    + SnapshotWriterAccess<A>
{
    type Uow: UnitOfWork;

    async fn find(&self, uow: &mut Self::Uow, id: A::Id) -> Result<Option<A>, RepositoryError<A>>
    where
        <Self as SnapshotReaderAccess<A>>::Reader: SnapshotReader<A, Uow = Self::Uow>,
        <Self as EventReaderAccess<A>>::Reader: EventReader<A, Uow = Self::Uow>,
    {
        self.find_at_version(uow, id, None).await
    }

    async fn find_at_version(
        &self,
        uow: &mut Self::Uow,
        id: A::Id,
        at: Option<AggregateVersion>,
    ) -> Result<Option<A>, RepositoryError<A>>
    where
        <Self as SnapshotReaderAccess<A>>::Reader: SnapshotReader<A, Uow = Self::Uow>,
        <Self as EventReaderAccess<A>>::Reader: EventReader<A, Uow = Self::Uow>,
    {
        let snapshot = {
            let reader = self.snapshot_reader();
            reader.read_latest_snapshot(uow, id, at).await?
        };
        let events = {
            let reader = self.event_reader();
            let start = snapshot
                .as_ref()
                .map(|s| Bound::Excluded(s.aggregate_version()))
                .unwrap_or(Bound::Unbounded);
            let end = at.map(Bound::Included).unwrap_or(Bound::Unbounded);
            let range = AggregateVersionRange::new(start, end);
            reader.read_events(uow, id, range).await?
        };
        if events.is_empty() && snapshot.is_none() {
            return Ok(None);
        }
        let mut aggregate = A::default();
        aggregate
            .replay_events(events, snapshot)
            .map_err(RepositoryError::Aggregate)?;
        Ok(Some(aggregate))
    }

    async fn save(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        aggregate: &mut A,
    ) -> Result<(), RepositoryError<A>>
    where
        <Self as EventWriterAccess<A>>::Writer: EventWriter<A, Uow = Self::Uow>,
        <Self as SnapshotReaderAccess<A>>::Reader: SnapshotReader<A, Uow = Self::Uow>,
        <Self as SnapshotWriterAccess<A>>::Writer: SnapshotWriter<A, Uow = Self::Uow>,
    {
        let events = aggregate.uncommitted_events();
        {
            let writer = self.event_writer();
            writer
                .write_events_and_outbox(uow, request_context, events)
                .await?;
        }
        match self.config().snapshot_policy {
            SnapshotPolicy::Disabled => {}
            SnapshotPolicy::AtLeast { minimum_interval } => {
                let aggregate_id = aggregate.aggregate_id().ok_or_else(|| {
                    let err: AggregateError<A::Id> = AggregateError::NoState;
                    RepositoryError::Aggregate(err.into())
                })?;
                let current_version = aggregate.version().as_u64();
                let latest_snapshot_version = {
                    let reader = self.snapshot_reader();
                    reader
                        .read_latest_snapshot(uow, aggregate_id, None)
                        .await?
                        .as_ref()
                        .map(|snapshot| snapshot.aggregate_version().as_u64())
                        .unwrap_or(0)
                };
                if current_version.saturating_sub(latest_snapshot_version)
                    >= minimum_interval.as_u64()
                {
                    let snapshot = aggregate
                        .to_snapshot()
                        .map_err(RepositoryError::Aggregate)?;
                    let writer = self.snapshot_writer();
                    writer.write_snapshot(uow, &snapshot).await?;
                }
            }
        }
        aggregate.clear_uncommitted_events();
        Ok(())
    }
}
