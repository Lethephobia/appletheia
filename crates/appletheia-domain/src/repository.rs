pub mod repository_error;

pub use crate::repository_error::{PersistenceErrorKind, RepositoryError};

use crate::aggregate::{Aggregate, AggregateVersion};
use crate::event::Event;
use crate::snapshot::Snapshot;

#[allow(async_fn_in_trait)]
pub trait Repository<A: Aggregate> {
    async fn read_events(
        &self,
        aggregate_id: A::Id,
    ) -> Result<
        (
            Vec<Event<A::Id, A::EventPayload>>,
            Option<Snapshot<A::State>>,
        ),
        RepositoryError<A>,
    >;

    async fn read_events_at_version(
        &self,
        aggregate_id: A::Id,
        version_at: AggregateVersion,
    ) -> Result<
        (
            Vec<Event<A::Id, A::EventPayload>>,
            Option<Snapshot<A::State>>,
        ),
        RepositoryError<A>,
    >;

    async fn find(&self, id: A::Id) -> Result<Option<A>, RepositoryError<A>> {
        let (events, snapshot) = self.read_events(id).await?;
        if events.is_empty() && snapshot.is_none() {
            return Ok(None);
        }
        let mut aggregate = A::default();
        aggregate
            .replay_events(events, snapshot)
            .map_err(RepositoryError::Aggregate)?;
        Ok(Some(aggregate))
    }

    async fn find_at_version(
        &self,
        id: A::Id,
        version_at: AggregateVersion,
    ) -> Result<Option<A>, RepositoryError<A>> {
        let (events, snapshot) = self.read_events_at_version(id, version_at).await?;
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
