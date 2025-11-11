pub mod repository_error;

pub use repository_error::RepositoryError;

use crate::aggregate::{Aggregate, AggregateVersion};
use crate::event::Event;
use crate::snapshot::Snapshot;

#[allow(async_fn_in_trait)]
pub trait Repository<A: Aggregate> {
    async fn read_latest_snapshot(
        &mut self,
        aggregate_id: A::Id,
        as_of: Option<AggregateVersion>,
    ) -> Result<Option<Snapshot<A::State>>, RepositoryError<A>>;

    async fn read_events(
        &mut self,
        aggregate_id: A::Id,
        after: Option<AggregateVersion>,
        as_of: Option<AggregateVersion>,
    ) -> Result<Vec<Event<A::Id, A::EventPayload>>, RepositoryError<A>>;

    async fn find(&mut self, id: A::Id) -> Result<Option<A>, RepositoryError<A>> {
        self.find_at_version(id, None).await
    }

    async fn find_at_version(
        &mut self,
        id: A::Id,
        at: Option<AggregateVersion>,
    ) -> Result<Option<A>, RepositoryError<A>> {
        let snapshot = self.read_latest_snapshot(id, at).await?;
        let events = self
            .read_events(id, snapshot.as_ref().map(|s| s.aggregate_version()), at)
            .await?;
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
