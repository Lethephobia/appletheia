use sqlx::{Postgres, QueryBuilder};

use appletheia_application::event::{
    EventEnvelope, EventFeedBatchSize, EventFeedReader, EventFeedReaderError, EventSelector,
    EventSequence,
};
use appletheia_application::messaging::Subscription;

use crate::postgresql::event::{PgEventRow, PgEventRowError};
use crate::postgresql::unit_of_work::PgUnitOfWork;

#[derive(Debug)]
pub struct PgEventFeedReader;

impl PgEventFeedReader {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgEventFeedReader {
    fn default() -> Self {
        Self::new()
    }
}

impl EventFeedReader for PgEventFeedReader {
    type Uow = PgUnitOfWork;

    async fn read_after(
        &self,
        uow: &mut Self::Uow,
        after: Option<EventSequence>,
        limit: EventFeedBatchSize,
        subscription: Subscription<'_, EventSelector>,
    ) -> Result<Vec<EventEnvelope>, EventFeedReaderError> {
        let selectors = match subscription {
            Subscription::All => None,
            Subscription::Only([]) => {
                return Err(EventFeedReaderError::InvalidSubscription);
            }
            Subscription::Only(selectors) => Some(selectors),
        };

        let mut query: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            SELECT
                event_sequence, id, aggregate_type, aggregate_id, aggregate_version,
                event_name, payload, occurred_at, correlation_id, causation_id, context
            FROM events
            "#,
        );

        let mut has_where = false;

        if let Some(after) = after {
            query
                .push(" WHERE event_sequence > ")
                .push_bind(after.value());
            has_where = true;
        }

        if let Some(selectors) = selectors {
            query.push(if has_where { " AND (" } else { " WHERE (" });
            let mut separated = query.separated(" OR ");
            for selector in selectors {
                separated
                    .push("(aggregate_type = ")
                    .push_bind(selector.aggregate_type.value())
                    .push(" AND event_name = ")
                    .push_bind(selector.event_name.value())
                    .push(")");
            }
            separated.push_unseparated(")");
        }

        query
            .push(" ORDER BY event_sequence ASC LIMIT ")
            .push_bind(limit.as_i64());

        let transaction = uow.transaction_mut();

        let event_rows = query
            .build_query_as::<PgEventRow>()
            .fetch_all(transaction.as_mut())
            .await
            .map_err(|e| EventFeedReaderError::Persistence(Box::new(e)))?;

        let envelopes = event_rows
            .into_iter()
            .map(|row| row.try_into_event_envelope())
            .collect::<Result<Vec<_>, PgEventRowError>>()
            .map_err(|e| EventFeedReaderError::Persistence(Box::new(e)))?;

        Ok(envelopes)
    }
}
