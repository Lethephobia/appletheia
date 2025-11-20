pub mod repository_error;

pub use repository_error::RepositoryError;

use crate::aggregate::{Aggregate, AggregateVersion};
use crate::event::EventReader;
use crate::snapshot::SnapshotReader;

#[allow(async_fn_in_trait)]
pub trait Repository<A: Aggregate> {
    type EventReader<'c>: EventReader<A>
    where
        Self: 'c;

    type SnapshotReader<'c>: SnapshotReader<A>
    where
        Self: 'c;

    fn event_reader(&mut self) -> Self::EventReader<'_>;

    fn snapshot_reader(&mut self) -> Self::SnapshotReader<'_>;

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
            reader
                .read_events(id, snapshot.as_ref().map(|s| s.aggregate_version()), at)
                .await?
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
