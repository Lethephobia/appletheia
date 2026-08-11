use appletheia_application::event::EventEnvelope;
use appletheia_application::outbox::event::{
    EventOutboxEnqueueError, EventOutboxEnqueuer, EventOutboxId,
};
use chrono::{DateTime, Utc};
use sqlx::{Postgres, QueryBuilder};

use crate::postgresql::unit_of_work::PgUnitOfWork;

/// Persists event outbox entries in the repository transaction.
pub struct PgEventOutboxEnqueuer;

impl PgEventOutboxEnqueuer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgEventOutboxEnqueuer {
    fn default() -> Self {
        Self::new()
    }
}

impl EventOutboxEnqueuer for PgEventOutboxEnqueuer {
    type Uow = PgUnitOfWork;

    async fn enqueue_events(
        &self,
        uow: &mut Self::Uow,
        events: &[EventEnvelope],
    ) -> Result<(), EventOutboxEnqueueError> {
        if events.is_empty() {
            return Ok(());
        }

        let mut query = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO event_outbox (
                id, event_sequence, event_id, aggregate_type, aggregate_id,
                aggregate_version, event_name, payload, occurred_at,
                correlation_id, causation_id, context
            ) VALUES
            "#,
        );
        let mut separated = query.separated(", ");
        for event in events {
            let context = serde_json::to_value(&event.context)
                .map_err(|error| EventOutboxEnqueueError::Persistence(Box::new(error)))?;

            separated
                .push("(")
                .push_bind(EventOutboxId::new().value())
                .push_bind(event.event_sequence.value())
                .push_bind(event.event_id.value())
                .push_bind(event.aggregate_type.value())
                .push_bind(event.aggregate_id.value())
                .push_bind(event.aggregate_version.value())
                .push_bind(event.event_name.value())
                .push_bind(event.payload.value().clone())
                .push_bind(DateTime::<Utc>::from(event.occurred_at))
                .push_bind(event.correlation_id.value())
                .push_bind(event.causation_id.value())
                .push_bind(context)
                .push(")");
        }

        query
            .build()
            .execute(uow.transaction_mut().as_mut())
            .await
            .map_err(|error| EventOutboxEnqueueError::Persistence(Box::new(error)))?;

        Ok(())
    }
}
