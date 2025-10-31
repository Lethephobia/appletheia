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
    ) -> Result<Event<A::Id, A::EventPayload>, RepositoryError<A, sqlx::Error>> {
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
    ) -> Result<Snapshot<A::State>, RepositoryError<A, sqlx::Error>> {
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
            CreatedAt::from(snapshot.created_at),
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
        RepositoryError<A, sqlx::Error>,
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
        .bind(aggregate_id.value().value())
        .fetch_optional(&self.pool)
        .await
        .map_err(RepositoryError::Persistence)?;
        let snapshot = snapshot_row.map(|row| self.map_snapshot(row)).transpose()?;

        let version_value_after = snapshot
            .as_ref()
            .map(|s| s.aggregate_version().value())
            .unwrap_or(0);

        let event_rows = sqlx::query_as::<_, PgEventModel>(
            r#"
            SELECT id, aggregate_type, aggregate_id, aggregate_version, payload, created_at
            FROM events WHERE aggregate_type = $1 AND aggregate_id = $2 AND aggregate_version > $3
            ORDER BY aggregate_version ASC
            "#,
        )
        .bind(A::AGGREGATE_TYPE)
        .bind(aggregate_id.value().value())
        .bind(version_value_after)
        .fetch_all(&self.pool)
        .await
        .map_err(RepositoryError::Persistence)?;

        let events = event_rows
            .into_iter()
            .map(|row| self.map_event(row))
            .collect::<Result<Vec<Event<A::Id, A::EventPayload>>, RepositoryError<A, sqlx::Error>>>(
            )?;

        Ok((events, snapshot))
    }

    async fn read_events_at_version(
        &self,
        aggregate_id: A::Id,
        version_at: AggregateVersion,
    ) -> Result<
        (
            Vec<Event<A::Id, A::EventPayload>>,
            Option<Snapshot<A::State>>,
        ),
        RepositoryError<A, sqlx::Error>,
    > {
        let snapshot_row = sqlx::query_as::<_, PgSnapshotModel>(
            r#"
            SELECT id, aggregate_type, aggregate_id, aggregate_version, state, created_at
            FROM snapshots WHERE aggregate_type = $1 AND aggregate_id = $2 AND aggregate_version <= $3
            ORDER BY aggregate_version DESC
            LIMIT 1
            "#,
        )
        .bind(A::AGGREGATE_TYPE)
        .bind(aggregate_id.value().value())
        .bind(version_at.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(RepositoryError::Persistence)?;
        let snapshot = snapshot_row.map(|row| self.map_snapshot(row)).transpose()?;

        let version_value_after = snapshot
            .as_ref()
            .map(|s| s.aggregate_version().value())
            .unwrap_or(0);

        let event_rows = sqlx::query_as::<_, PgEventModel>(
            r#"
            SELECT id, aggregate_type, aggregate_id, aggregate_version, payload, created_at
            FROM events WHERE aggregate_type = $1 AND aggregate_id = $2
                AND aggregate_version > $3 AND aggregate_version <= $4
            ORDER BY aggregate_version ASC
            "#,
        )
        .bind(A::AGGREGATE_TYPE)
        .bind(aggregate_id.value().value())
        .bind(version_value_after)
        .bind(version_at.value())
        .fetch_all(&self.pool)
        .await
        .map_err(RepositoryError::Persistence)?;

        let events = event_rows
            .into_iter()
            .map(|row| self.map_event(row))
            .collect::<Result<Vec<Event<A::Id, A::EventPayload>>, RepositoryError<A, sqlx::Error>>>(
            )?;

        Ok((events, snapshot))
    }
}

impl<A: Aggregate> Repository<A, sqlx::Error> for PgRepository<A> {
    async fn find(&self, id: A::Id) -> Result<Option<A>, RepositoryError<A, sqlx::Error>> {
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
    ) -> Result<Option<A>, RepositoryError<A, sqlx::Error>> {
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
