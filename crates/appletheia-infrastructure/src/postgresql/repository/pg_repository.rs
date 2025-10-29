use sqlx::PgPool;

use crate::postgresql::event::PgEventModel;
use crate::postgresql::snapshot::PgSnapshotModel;
use appletheia_domain::{
    Aggregate, AggregateId, AggregateState, AggregateVersion, CreatedAt, Event, EventId,
    EventPayload, Repository, RepositoryError, Snapshot, SnapshotId,
};

use std::marker::PhantomData;

pub struct PgRepository<A: Aggregate> {
    pool: PgPool,
    _phantom: PhantomData<A>,
}

impl<A: Aggregate> PgRepository<A> {
    fn map_event(
        &self,
        event: PgEventModel,
    ) -> Result<Event<A::Id, A::EventPayload>, RepositoryError<A>> {
        let id = EventId::try_from(event.id).map_err(RepositoryError::EventId)?;
        let aggregate_id =
            A::Id::try_from_uuid(event.aggregate_id).map_err(RepositoryError::AggregateId)?;
        let aggregate_version = AggregateVersion::try_from(event.aggregate_version)
            .map_err(RepositoryError::AggregateVersion)?;
        let payload = A::EventPayload::try_from_json_value(event.payload)
            .map_err(RepositoryError::EventPayload)?;
        Ok(Event::from_persisted(
            id,
            aggregate_id,
            aggregate_version,
            payload,
            CreatedAt::from(event.created_at),
        ))
    }

    fn map_snapshot(
        &self,
        snapshot: PgSnapshotModel,
    ) -> Result<Snapshot<A::State>, RepositoryError<A>> {
        let id = SnapshotId::try_from(snapshot.id).map_err(RepositoryError::SnapshotId)?;
        let aggregate_id =
            A::Id::try_from_uuid(snapshot.aggregate_id).map_err(RepositoryError::AggregateId)?;
        let aggregate_version = AggregateVersion::try_from(snapshot.aggregate_version)
            .map_err(RepositoryError::AggregateVersion)?;
        let state = A::State::try_from_json_value(snapshot.state)
            .map_err(RepositoryError::AggregateState)?;
        Ok(Snapshot::from_persisted(
            id,
            aggregate_id,
            aggregate_version,
            state,
            snapshot.created_at,
        ))
    }

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
        let snapshot_row = sqlx::query_as::<_, PgSnapshotModel>(
            r#"
            SELECT id, aggregate_type, aggregate_id, aggregate_version, state, created_at
            FROM snapshots WHERE aggregate_type = $1 AND aggregate_id = $2
            ORDER BY aggregate_version DESC
            LIMIT 1
            "#,
        )
        .bind(A::AGGREGATE_TYPE)
        .bind(AggregateId::value(aggregate_id).value())
        .fetch_optional(&self.pool)
        .await
        .map_err(RepositoryError::Database)?;
        let snapshot =
            snapshot_row.map(|row| self.map_snapshot(row).map_err(RepositoryError::Snapshot)?);

        let events_rows = sqlx::query_as::<_, PgEventModel>(
            r#"
            SELECT id, aggregate_type, aggregate_id, aggregate_version, event_type, payload, created_at
            FROM events WHERE aggregate_type = $1 AND aggregate_id = $2 AND aggregate_version > $3
            ORDER BY aggregate_version ASC
            "#,
        )
        .bind(A::AGGREGATE_TYPE)
        .bind(AggregateId::value(aggregate_id).value())
        .bind(snapshot.map(|s| s.aggregate_version().value()).unwrap_or(0))
        .fetch_all(&self.pool)
        .await
        .map_err(RepositoryError::Database)?;

        let events = events_rows
            .iter()
            .map(|row| self.map_event(row).map_err(RepositoryError::EventPayload)?)
            .collect();
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
            .map_err(RepositoryError::Aggregate)?;
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
            .map_err(RepositoryError::Aggregate)?;
        Ok(Some(aggregate))
    }
}
