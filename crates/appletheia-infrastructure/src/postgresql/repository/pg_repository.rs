use sqlx::PgPool;

use crate::postgresql::event::PgEventModel;
use appletheia_domain::{
    Aggregate, AggregateVersion, Event, Repository, RepositoryError, Snapshot,
};

use std::marker::PhantomData;

pub struct PgRepository<A: Aggregate> {
    pool: PgPool,
    _phantom: PhantomData<A>,
}

impl<A: Aggregate> PgRepository<A> {
    async fn read_events(
        &self,
        aggregate_id: A::Id,
    ) -> Result<
        (
            Vec<Event<A::Id, A::EventPayload>>,
            Option<Snapshot<A::State>>,
        ),
        RepositoryError<A>,
    > {
        todo!()
    }

    async fn read_events_at_version(
        &self,
        aggregate_id: A::Id,
        version: AggregateVersion,
    ) -> Result<
        (
            Vec<Event<A::Id, A::EventPayload>>,
            Option<Snapshot<A::State>>,
        ),
        RepositoryError<A>,
    > {
        todo!()
    }
}

impl<A: Aggregate> Repository<A> for PgRepository<A> {
    async fn find(&self, id: A::Id) -> Result<Option<A>, RepositoryError<A>> {
        let (events, snapshot) = self.read_events(id).await?;
        if events.is_empty() && snapshot.is_none() {
            return Ok(None);
        }
        let mut aggregate = A::default();
        aggregate
            .replay_events(events, snapshot)
            .map_err(RepositoryError::AggregateError)?;
        Ok(Some(aggregate))
    }

    async fn find_at_version(
        &self,
        id: A::Id,
        version: AggregateVersion,
    ) -> Result<Option<A>, RepositoryError<A>> {
        let (events, snapshot) = self.read_events_at_version(id, version).await?;
        if events.is_empty() && snapshot.is_none() {
            return Ok(None);
        }
        let mut aggregate = A::default();
        aggregate
            .replay_events(events, snapshot)
            .map_err(RepositoryError::AggregateError)?;
        Ok(Some(aggregate))
    }
}
