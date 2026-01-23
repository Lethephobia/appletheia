use std::marker::PhantomData;

use appletheia_application::{
    event::{EventWriter, EventWriterError},
    outbox::OrderingKey,
    outbox::event::EventOutboxId,
    request_context::RequestContext,
    unit_of_work::UnitOfWorkError,
};
use appletheia_domain::{Aggregate, AggregateId, Event};
use chrono::{DateTime, Utc};
use sqlx::{Postgres, QueryBuilder};

use crate::postgresql::unit_of_work::PgUnitOfWork;

use super::{PgEventRow, PgEventRowError};

pub struct PgEventWriter<A: Aggregate> {
    _aggregate: PhantomData<A>,
}

impl<A: Aggregate> PgEventWriter<A> {
    pub fn new() -> Self {
        Self {
            _aggregate: PhantomData,
        }
    }
}

impl<A: Aggregate> Default for PgEventWriter<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: Aggregate> EventWriter<A> for PgEventWriter<A> {
    type Uow = PgUnitOfWork;

    async fn write_events_and_outbox(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        events: &[Event<A::Id, A::EventPayload>],
    ) -> Result<(), EventWriterError> {
        if events.is_empty() {
            return Ok(());
        }

        let correlation_id = request_context.correlation_id.0;
        let causation_id = request_context.message_id.value();
        let context_json = serde_json::to_value(request_context).map_err(EventWriterError::Json)?;

        let mut events_query = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO events (
                id, aggregate_type, aggregate_id, aggregate_version,
                payload, occurred_at, correlation_id, causation_id, context
            ) VALUES
            "#,
        );

        let mut sep = events_query.separated(", ");
        for event in events {
            let id = event.id().value();
            let aggregate_id = event.aggregate_id().value();
            let version = event.aggregate_version().value();
            let payload = serde_json::to_value(event.payload()).map_err(EventWriterError::Json)?;
            let occurred_at: DateTime<Utc> = event.occurred_at().into();

            sep.push("(")
                .push_bind(id)
                .push_bind(A::AGGREGATE_TYPE.to_string())
                .push_bind(aggregate_id)
                .push_bind(version)
                .push_bind(payload)
                .push_bind(occurred_at)
                .push_bind(correlation_id)
                .push_bind(causation_id)
                .push_bind(&context_json)
                .push(")");
        }
        events_query.push(
            r#"
            RETURNING
                event_sequence,
                id,
                aggregate_type,
                aggregate_id,
                aggregate_version,
                payload,
                occurred_at,
                correlation_id,
                causation_id,
                context
            "#,
        );

        let transaction = uow.transaction_mut().map_err(|e| match e {
            UnitOfWorkError::NotInTransaction => EventWriterError::NotInTransaction,
            other => EventWriterError::Persistence(Box::new(other)),
        })?;

        let event_rows = events_query
            .build_query_as::<PgEventRow>()
            .fetch_all(transaction.as_mut())
            .await
            .map_err(|e| EventWriterError::Persistence(Box::new(e)))?;

        let mut outbox_query = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO event_outbox (
                id, event_sequence, event_id, aggregate_type, aggregate_id,
                aggregate_version, ordering_key, payload, occurred_at,
                correlation_id, causation_id, context
            ) VALUES
            "#,
        );
        let mut sep = outbox_query.separated(", ");
        for event_row in event_rows {
            let outbox_id = EventOutboxId::new().value();
            let app_event = event_row
                .try_into_app_event::<A::AggregateType>()
                .map_err(|e: PgEventRowError| EventWriterError::Persistence(Box::new(e)))?;
            let ordering_key =
                OrderingKey::from((&app_event.aggregate_type, &app_event.aggregate_id)).to_string();

            sep.push("(")
                .push_bind(outbox_id)
                .push_bind(app_event.event_sequence.value())
                .push_bind(app_event.event_id.value())
                .push_bind(app_event.aggregate_type.to_string())
                .push_bind(app_event.aggregate_id.value())
                .push_bind(app_event.aggregate_version.value())
                .push_bind(ordering_key)
                .push_bind(app_event.payload.value().clone())
                .push_bind(DateTime::<Utc>::from(app_event.occurred_at))
                .push_bind(app_event.correlation_id.0)
                .push_bind(app_event.causation_id.value())
                .push_bind(&context_json)
                .push(")");
        }
        outbox_query
            .build()
            .execute(transaction.as_mut())
            .await
            .map_err(|e| EventWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }
}
