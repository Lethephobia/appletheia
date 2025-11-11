use sqlx::{Postgres, QueryBuilder, Transaction};

use crate::postgresql::event::{PgEventRow, PgEventRowError};
use crate::postgresql::snapshot::PgSnapshotRow;
use appletheia_domain::{
    Aggregate, AggregateId, AggregateVersion, Event, Repository, RepositoryError, Snapshot,
};

use std::marker::PhantomData;

pub struct PgRepository<'c, A: Aggregate> {
    transaction: &'c mut Transaction<'static, Postgres>,
    _phantom: PhantomData<A>,
}

impl<'c, A: Aggregate> PgRepository<'c, A> {
    pub fn new(transaction: &'c mut Transaction<'static, Postgres>) -> Self {
        Self {
            transaction,
            _phantom: PhantomData,
        }
    }
}

impl<'c, A: Aggregate> Repository<A> for PgRepository<'c, A> {
    async fn read_latest_snapshot(
        &mut self,
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
            .fetch_optional(self.transaction.as_mut())
            .await
            .map_err(|e| RepositoryError::Persistence(Box::new(e)))?;
        let snapshot = snapshot_row
            .map(|row| row.try_into_snapshot::<A>())
            .transpose()
            .map_err(|e| RepositoryError::MappingFailed(Box::new(e)))?;
        Ok(snapshot)
    }

    async fn read_events(
        &mut self,
        aggregate_id: A::Id,
        after: Option<AggregateVersion>,
        as_of: Option<AggregateVersion>,
    ) -> Result<Vec<Event<A::Id, A::EventPayload>>, RepositoryError<A>> {
        if let (Some(a), Some(u)) = (after, as_of)
            && a >= u
        {
            return Ok(Vec::new());
        }

        let mut query: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            SELECT
                event_sequence, id, aggregate_type, aggregate_id, aggregate_version,
                payload, occurred_at, correlation_id, causation_id, context
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
            .fetch_all(self.transaction.as_mut())
            .await
            .map_err(|e| RepositoryError::Persistence(Box::new(e)))?;

        let events = event_rows
            .into_iter()
            .map(|row| row.try_into_event::<A>())
            .collect::<Result<Vec<Event<A::Id, A::EventPayload>>, PgEventRowError<A>>>()
            .map_err(|e| RepositoryError::MappingFailed(Box::new(e)))?;

        Ok(events)
    }
}
