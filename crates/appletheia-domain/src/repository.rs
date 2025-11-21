pub mod repository_error;

pub use repository_error::RepositoryError;

use crate::aggregate::{Aggregate, AggregateVersion, AggregateVersionRange};
use crate::event::EventReader;
use crate::snapshot::SnapshotReader;
use std::ops::Bound;

pub trait EventReaderProvider<A: Aggregate> {
    type EventReader<'c>: EventReader<A>
    where
        Self: 'c;

    fn event_reader(&mut self) -> Self::EventReader<'_>;
}

pub trait SnapshotReaderProvider<A: Aggregate> {
    type SnapshotReader<'c>: SnapshotReader<A>
    where
        Self: 'c;

    fn snapshot_reader(&mut self) -> Self::SnapshotReader<'_>;
}

#[allow(async_fn_in_trait)]
pub trait Repository<A: Aggregate>: EventReaderProvider<A> + SnapshotReaderProvider<A> {
    async fn find(&mut self, id: A::Id) -> Result<Option<A>, RepositoryError<A>> {
        self.find_at_version(id, None).await
    }

    async fn find_at_version(
        &mut self,
        id: A::Id,
        at: Option<AggregateVersion>,
    ) -> Result<Option<A>, RepositoryError<A>> {
        let snapshot = {
            let mut reader = self.snapshot_reader();
            reader.read_latest_snapshot(id, at).await?
        };
        let events = {
            let mut reader = self.event_reader();
            let start = snapshot
                .as_ref()
                .map(|s| Bound::Excluded(s.aggregate_version()))
                .unwrap_or(Bound::Unbounded);
            let end = at.map(Bound::Included).unwrap_or(Bound::Unbounded);
            let range = AggregateVersionRange::new(start, end);
            reader.read_events(id, range).await?
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
}
