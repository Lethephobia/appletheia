use sqlx::Postgres;

use appletheia_application::event::{EventSequence, EventSequenceError};
use appletheia_application::event::{EventSequenceLookup, EventSequenceLookupError};
use appletheia_application::request_context::CausationId;

use crate::postgresql::unit_of_work::PgUnitOfWork;

#[derive(Debug)]
pub struct PgEventSequenceLookup;

impl PgEventSequenceLookup {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgEventSequenceLookup {
    fn default() -> Self {
        Self::new()
    }
}

impl EventSequenceLookup for PgEventSequenceLookup {
    type Uow = PgUnitOfWork;

    async fn max_event_sequence_by_causation_id(
        &self,
        uow: &mut Self::Uow,
        causation_id: CausationId,
    ) -> Result<Option<EventSequence>, EventSequenceLookupError> {
        let transaction = uow.transaction_mut();

        let row: (Option<i64>,) = sqlx::query_as::<Postgres, (Option<i64>,)>(
            r#"
            SELECT max(event_sequence)
              FROM events
             WHERE causation_id = $1
            "#,
        )
        .bind(causation_id.value())
        .fetch_one(transaction.as_mut())
        .await
        .map_err(|source| EventSequenceLookupError::Persistence(Box::new(source)))?;

        let Some(max) = row.0 else {
            return Ok(None);
        };

        let seq = EventSequence::try_from(max)
            .map_err(|e: EventSequenceError| EventSequenceLookupError::Persistence(Box::new(e)))?;

        Ok(Some(seq))
    }
}
