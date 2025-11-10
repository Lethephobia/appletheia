use sqlx::PgPool;

use crate::postgresql::event::{PgEventRow, PgEventRowError};
use crate::postgresql::snapshot::PgSnapshotRow;
use appletheia_domain::{
    Aggregate, AggregateId, AggregateVersion, Event, Repository, RepositoryError, Snapshot,
};

use std::marker::PhantomData;

pub struct PgRepository<A: Aggregate> {
    pool: PgPool,
    _phantom: PhantomData<A>,
}

impl<A: Aggregate> Repository<A> for PgRepository<A> {
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
        let snapshot_row = sqlx::query_as::<_, PgSnapshotRow>(
            r#"
            SELECT id, aggregate_type, aggregate_id, aggregate_version, state, materialized_at
            FROM snapshots WHERE aggregate_type = $1 AND aggregate_id = $2
            ORDER BY aggregate_version DESC
            LIMIT 1
            "#,
        )
        .bind(A::AGGREGATE_TYPE)
        .bind(aggregate_id.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::Persistence(Box::new(e)))?;
        let snapshot = snapshot_row
            .map(|row| row.to_snapshot::<A>())
            .transpose()
            .map_err(|e| RepositoryError::MappingFailed(Box::new(e)))?;

        let version_value_after = snapshot
            .as_ref()
            .map(|s| s.aggregate_version().value())
            .unwrap_or(0);

        let event_rows = sqlx::query_as::<_, PgEventRow>(
            r#"
            SELECT
                event_sequence, id, aggregate_type, aggregate_id, aggregate_version,
                payload, occurred_at, recorded_at, correlation_id, causation_id, context
            FROM events WHERE aggregate_type = $1 AND aggregate_id = $2 AND aggregate_version > $3
            ORDER BY aggregate_version ASC
            "#,
        )
        .bind(A::AGGREGATE_TYPE)
        .bind(aggregate_id.value())
        .bind(version_value_after)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Persistence(Box::new(e)))?;

        let events =
            event_rows
                .into_iter()
                .map(|row| row.to_event::<A>())
                .collect::<Result<
                    Vec<Event<A::Id, A::EventPayload>>,
                    PgEventRowError<A::Id, A::EventPayload>,
                >>().map_err(|e| RepositoryError::MappingFailed(Box::new(e)))?;

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
        RepositoryError<A>,
    > {
        let snapshot_row = sqlx::query_as::<_, PgSnapshotRow>(
            r#"
            SELECT id, aggregate_type, aggregate_id, aggregate_version, state, materialized_at
            FROM snapshots WHERE aggregate_type = $1 AND aggregate_id = $2 AND aggregate_version <= $3
            ORDER BY aggregate_version DESC
            LIMIT 1
            "#,
        )
        .bind(A::AGGREGATE_TYPE)
        .bind(aggregate_id.value())
        .bind(version_at.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::Persistence(Box::new(e)))?;
        let snapshot = snapshot_row
            .map(|row| row.to_snapshot::<A>())
            .transpose()
            .map_err(|e| RepositoryError::MappingFailed(Box::new(e)))?;

        let version_value_after = snapshot
            .as_ref()
            .map(|s| s.aggregate_version().value())
            .unwrap_or(0);

        let event_rows = sqlx::query_as::<_, PgEventRow>(
            r#"
            SELECT
                event_sequence, id, aggregate_type, aggregate_id, aggregate_version,
                payload, occurred_at, recorded_at, correlation_id, causation_id, context
            FROM events WHERE aggregate_type = $1 AND aggregate_id = $2
                AND aggregate_version > $3 AND aggregate_version <= $4
            ORDER BY aggregate_version ASC
            "#,
        )
        .bind(A::AGGREGATE_TYPE)
        .bind(aggregate_id.value())
        .bind(version_value_after)
        .bind(version_at.value())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Persistence(Box::new(e)))?;

        let events = event_rows
            .into_iter()
            .map(|row| row.to_event::<A>())
            .collect::<Result<Vec<Event<A::Id, A::EventPayload>>, PgEventRowError<A::Id, A::EventPayload>>>()
            .map_err(|e| RepositoryError::MappingFailed(Box::new(e)))?;

        Ok((events, snapshot))
    }
}
