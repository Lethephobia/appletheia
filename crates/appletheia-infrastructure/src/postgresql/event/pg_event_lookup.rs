use sqlx::Postgres;

use appletheia_application::event::{
    EventEnvelope, EventLookup, EventLookupError, EventSequence, EventSequenceError,
};
use appletheia_application::request_context::{CausationId, CorrelationId};
use appletheia_domain::EventId;

use crate::postgresql::event::PgEventRow;
use crate::postgresql::unit_of_work::PgUnitOfWork;

#[derive(Debug)]
pub struct PgEventLookup;

impl PgEventLookup {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgEventLookup {
    fn default() -> Self {
        Self::new()
    }
}

impl EventLookup for PgEventLookup {
    type Uow = PgUnitOfWork;

    async fn max_event_sequence_by_causation_id(
        &self,
        uow: &mut Self::Uow,
        causation_id: CausationId,
    ) -> Result<Option<EventSequence>, EventLookupError> {
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
        .map_err(|source| EventLookupError::Persistence(Box::new(source)))?;

        let Some(max) = row.0 else {
            return Ok(None);
        };

        let seq = EventSequence::try_from(max)
            .map_err(|e: EventSequenceError| EventLookupError::Persistence(Box::new(e)))?;

        Ok(Some(seq))
    }

    async fn last_event_id_by_causation_id(
        &self,
        uow: &mut Self::Uow,
        causation_id: CausationId,
    ) -> Result<Option<EventId>, EventLookupError> {
        let transaction = uow.transaction_mut();

        let row: Option<(uuid::Uuid,)> = sqlx::query_as::<Postgres, (uuid::Uuid,)>(
            r#"
            SELECT event_id
              FROM events
             WHERE causation_id = $1
             ORDER BY aggregate_version DESC
             LIMIT 1
            "#,
        )
        .bind(causation_id.value())
        .fetch_optional(transaction.as_mut())
        .await
        .map_err(|source| EventLookupError::Persistence(Box::new(source)))?;

        let Some(row) = row else {
            return Ok(None);
        };

        let event_id =
            EventId::try_from(row.0).map_err(|e| EventLookupError::Persistence(Box::new(e)))?;

        Ok(Some(event_id))
    }

    async fn events_by_causation_id(
        &self,
        uow: &mut Self::Uow,
        causation_id: CausationId,
    ) -> Result<Vec<EventEnvelope>, EventLookupError> {
        let transaction = uow.transaction_mut();

        let rows: Vec<PgEventRow> = sqlx::query_as::<Postgres, PgEventRow>(
            r#"
            SELECT
              event_sequence,
              id,
              aggregate_type,
              aggregate_id,
              aggregate_version,
              event_name,
              payload,
              occurred_at,
              correlation_id,
              causation_id,
              context
              FROM events
             WHERE causation_id = $1
             ORDER BY event_sequence ASC
            "#,
        )
        .bind(causation_id.value())
        .fetch_all(transaction.as_mut())
        .await
        .map_err(|source| EventLookupError::Persistence(Box::new(source)))?;

        rows.into_iter()
            .map(PgEventRow::try_into_event_envelope)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|source| EventLookupError::MappingFailed(Box::new(source)))
    }

    async fn events_by_correlation_id(
        &self,
        uow: &mut Self::Uow,
        correlation_id: CorrelationId,
    ) -> Result<Vec<EventEnvelope>, EventLookupError> {
        let transaction = uow.transaction_mut();

        let rows: Vec<PgEventRow> = sqlx::query_as::<Postgres, PgEventRow>(
            r#"
            SELECT
              event_sequence,
              id,
              aggregate_type,
              aggregate_id,
              aggregate_version,
              event_name,
              payload,
              occurred_at,
              correlation_id,
              causation_id,
              context
              FROM events
             WHERE correlation_id = $1
             ORDER BY event_sequence ASC
            "#,
        )
        .bind(correlation_id.value())
        .fetch_all(transaction.as_mut())
        .await
        .map_err(|source| EventLookupError::Persistence(Box::new(source)))?;

        rows.into_iter()
            .map(PgEventRow::try_into_event_envelope)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|source| EventLookupError::MappingFailed(Box::new(source)))
    }
}
