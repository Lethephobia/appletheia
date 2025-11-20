use std::marker::PhantomData;

use sqlx::{Postgres, QueryBuilder, Transaction};

use appletheia_domain::{
    Aggregate, AggregateId, AggregateVersion, Event, EventReader, EventReaderError,
};

use crate::postgresql::event::{PgEventRow, PgEventRowError};

pub struct PgEventReader<'c, A: Aggregate> {
    transaction: &'c mut Transaction<'static, Postgres>,
    _phantom: PhantomData<A>,
}

impl<'c, A: Aggregate> PgEventReader<'c, A> {
    pub(crate) fn new(transaction: &'c mut Transaction<'static, Postgres>) -> Self {
        Self {
            transaction,
            _phantom: PhantomData,
        }
    }
}

impl<'c, A: Aggregate> EventReader<A> for PgEventReader<'c, A> {
    async fn read_events(
        &mut self,
        aggregate_id: A::Id,
        after: Option<AggregateVersion>,
        as_of: Option<AggregateVersion>,
    ) -> Result<Vec<Event<A::Id, A::EventPayload>>, EventReaderError> {
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
            .push_bind(A::AGGREGATE_TYPE.value())
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
            .map_err(|e| EventReaderError::Persistence(Box::new(e)))?;

        let events = event_rows
            .into_iter()
            .map(|row| row.try_into_event::<A>())
            .collect::<Result<Vec<Event<A::Id, A::EventPayload>>, PgEventRowError<A>>>()
            .map_err(|e| EventReaderError::MappingFailed(Box::new(e)))?;

        Ok(events)
    }
}
