use sqlx::Postgres;

use appletheia_application::event::{EventEnvelope, EventLookup, EventLookupError};
use appletheia_application::request_context::CausationId;
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

    async fn events_by_event_ids(
        &self,
        uow: &mut Self::Uow,
        event_ids: &[EventId],
    ) -> Result<Vec<EventEnvelope>, EventLookupError> {
        if event_ids.is_empty() {
            return Ok(Vec::new());
        }

        let transaction = uow.transaction_mut();
        let event_id_values: Vec<uuid::Uuid> =
            event_ids.iter().map(|event_id| event_id.value()).collect();

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
             WHERE id = ANY($1)
             ORDER BY event_sequence ASC
            "#,
        )
        .bind(event_id_values)
        .fetch_all(transaction.as_mut())
        .await
        .map_err(|source| EventLookupError::Persistence(Box::new(source)))?;

        rows.into_iter()
            .map(PgEventRow::try_into_event_envelope)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|source| EventLookupError::MappingFailed(Box::new(source)))
    }
}
