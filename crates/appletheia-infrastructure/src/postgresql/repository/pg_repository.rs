use sqlx::{PgPool, Postgres, QueryBuilder};

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
    async fn read_latest_snapshot(
        &self,
        aggregate_id: A::Id,
        as_of: Option<AggregateVersion>,
    ) -> Result<Option<Snapshot<A::State>>, RepositoryError<A>> {
        let mut query: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            SELECT id, aggregate_type, aggregate_id, aggregate_version, state, materialized_at
            FROM snapshots WHERE aggregate_type = "#,
        );
        query
            .push_bind(A::AGGREGATE_TYPE)
            .push(" AND aggregate_id = ")
            .push_bind(aggregate_id.value());

        if let Some(version) = as_of {
            query
                .push(" AND aggregate_version <= ")
                .push_bind(version.value());
        }
        query.push(" ORDER BY aggregate_version DESC LIMIT 1");

        let snapshot_row = query
            .build_query_as::<PgSnapshotRow>()
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepositoryError::Persistence(Box::new(e)))?;
        let snapshot = snapshot_row
            .map(|row| row.to_snapshot::<A>())
            .transpose()
            .map_err(|e| RepositoryError::MappingFailed(Box::new(e)))?;
        Ok(snapshot)
    }

    async fn read_events(
        &self,
        aggregate_id: A::Id,
        after: Option<AggregateVersion>,
        as_of: Option<AggregateVersion>,
    ) -> Result<Vec<Event<A::Id, A::EventPayload>>, RepositoryError<A>> {
        if let (Some(a), Some(u)) = (after, as_of) {
            if a >= u {
                return Ok(Vec::new());
            }
        }

        let mut query: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            SELECT
                event_sequence, id, aggregate_type, aggregate_id, aggregate_version,
                payload, occurred_at, recorded_at, correlation_id, causation_id, context
            FROM events WHERE aggregate_type = "#,
        );

        query
            .push_bind(A::AGGREGATE_TYPE)
            .push(" AND aggregate_id = ")
            .push_bind(aggregate_id.value());

        if let Some(version) = after {
            query
                .push(" AND aggregate_version > ")
                .push_bind(version.value());
        }
        if let Some(version) = as_of {
            query
                .push(" AND aggregate_version <= ")
                .push_bind(version.value());
        }
        query.push(" ORDER BY aggregate_version ASC");

        let event_rows = query
            .build_query_as::<PgEventRow>()
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

        Ok(events)
    }
}
