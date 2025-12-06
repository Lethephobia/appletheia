use std::marker::PhantomData;
use std::ops::{Bound, RangeBounds};

use sqlx::{Postgres, QueryBuilder};

use appletheia_application::event::{EventReader, EventReaderError};
use appletheia_application::unit_of_work::UnitOfWorkError;
use appletheia_domain::{Aggregate, AggregateId, AggregateVersionRange, Event};

use crate::postgresql::event::{PgEventRow, PgEventRowError};
use crate::postgresql::unit_of_work::PgUnitOfWork;

pub struct PgEventReader<A: Aggregate> {
    _phantom: PhantomData<A>,
}

impl<A: Aggregate> PgEventReader<A> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<A: Aggregate> Default for PgEventReader<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: Aggregate> EventReader<A> for PgEventReader<A> {
    type Uow = PgUnitOfWork;

    async fn read_events(
        &self,
        uow: &mut Self::Uow,
        aggregate_id: A::Id,
        range: AggregateVersionRange,
    ) -> Result<Vec<Event<A::Id, A::EventPayload>>, EventReaderError> {
        if range_is_empty(&range) {
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
            .push_bind(A::AGGREGATE_TYPE.value())
            .push(" AND aggregate_id = ")
            .push_bind(aggregate_id.value());

        match range.start_bound() {
            Bound::Included(version) => {
                query
                    .push(" AND aggregate_version >= ")
                    .push_bind(version.value());
            }
            Bound::Excluded(version) => {
                query
                    .push(" AND aggregate_version > ")
                    .push_bind(version.value());
            }
            Bound::Unbounded => {}
        }
        match range.end_bound() {
            Bound::Included(version) => {
                query
                    .push(" AND aggregate_version <= ")
                    .push_bind(version.value());
            }
            Bound::Excluded(version) => {
                query
                    .push(" AND aggregate_version < ")
                    .push_bind(version.value());
            }
            Bound::Unbounded => {}
        }
        query.push(" ORDER BY aggregate_version ASC");

        let transaction = uow.transaction_mut().map_err(|e| match e {
            UnitOfWorkError::NotInTransaction => EventReaderError::NotInTransaction,
            other => EventReaderError::Persistence(Box::new(other)),
        })?;

        let event_rows = query
            .build_query_as::<PgEventRow>()
            .fetch_all(transaction.as_mut())
            .await
            .map_err(|e| EventReaderError::Persistence(Box::new(e)))?;

        let events = event_rows
            .into_iter()
            .map(|row| row.try_into_event::<A>())
            .collect::<Result<Vec<Event<A::Id, A::EventPayload>>, PgEventRowError<A>>>()
            .map_err(|e| EventReaderError::MappingFailed(Box::new(e)))?;

        Ok(events)
    }
}

fn range_is_empty(range: &AggregateVersionRange) -> bool {
    use Bound::*;
    match (range.start_bound(), range.end_bound()) {
        (_, Unbounded) | (Unbounded, _) => false,
        (Included(start), Included(end)) => start > end,
        (Included(start), Excluded(end)) => start >= end,
        (Excluded(start), Included(end)) => start >= end,
        (Excluded(start), Excluded(end)) => start >= end,
    }
}
