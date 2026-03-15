use chrono::{DateTime, Utc};
use sqlx::{Postgres, QueryBuilder};

use appletheia_application::messaging::PublishDispatchError;
use appletheia_application::outbox::{
    OutboxLifecycle, OutboxWriter, OutboxWriterError, event::EventOutbox,
};

use crate::postgresql::unit_of_work::PgUnitOfWork;

pub struct PgEventOutboxWriter;

impl PgEventOutboxWriter {
    pub fn new() -> Self {
        Self
    }

    fn serialize_last_error(
        outbox: &EventOutbox,
    ) -> Result<Option<serde_json::Value>, OutboxWriterError> {
        match &outbox.last_error {
            Some(error) => {
                let json = serde_json::to_value(error as &PublishDispatchError)
                    .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;
                Ok(Some(json))
            }
            None => Ok(None),
        }
    }

    async fn upsert_outbox_rows(
        uow: &mut PgUnitOfWork,
        outboxes: &[&EventOutbox],
    ) -> Result<(), OutboxWriterError> {
        if outboxes.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO event_outbox (
                id,
                event_sequence,
                event_id,
                aggregate_type,
                aggregate_id,
                aggregate_version,
                event_name,
                payload,
                occurred_at,
                correlation_id,
                causation_id,
                context,
                published_at,
                attempt_count,
                next_attempt_after,
                lease_owner,
                lease_until,
                last_error
            ) VALUES
            "#,
        );

        {
            let mut separated = query_builder.separated(", ");
            for outbox in outboxes {
                let event = &outbox.event;
                let context_value = serde_json::to_value(&event.context)
                    .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;
                let last_error_value = Self::serialize_last_error(outbox)?;
                let next_attempt_after_value = outbox
                    .state
                    .next_attempt_after()
                    .unwrap_or_default()
                    .value();

                separated
                    .push("(")
                    .push_bind(outbox.id.value())
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
                    .push_bind(context_value)
                    .push_bind(outbox.state.published_at().map(DateTime::<Utc>::from))
                    .push_bind(outbox.state.attempt_count().value())
                    .push_bind(next_attempt_after_value)
                    .push_bind(outbox.state.lease_owner().map(ToString::to_string))
                    .push_bind(outbox.state.lease_until().map(DateTime::<Utc>::from))
                    .push_bind(last_error_value)
                    .push(")");
            }
        }

        query_builder.push(
            r#"
            ON CONFLICT (id) DO UPDATE
               SET published_at = EXCLUDED.published_at,
                   attempt_count = EXCLUDED.attempt_count,
                   next_attempt_after = EXCLUDED.next_attempt_after,
                   lease_owner = EXCLUDED.lease_owner,
                   lease_until = EXCLUDED.lease_until,
                   last_error = EXCLUDED.last_error
            "#,
        );

        let transaction = uow.transaction_mut();

        query_builder
            .build()
            .execute(transaction.as_mut())
            .await
            .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;

        Ok(())
    }

    async fn insert_dead_letters(
        uow: &mut PgUnitOfWork,
        dead_lettered_outboxes: &[&EventOutbox],
    ) -> Result<(), OutboxWriterError> {
        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO event_dead_letters (
                event_outbox_id,
                event_sequence,
                event_id,
                aggregate_type,
                aggregate_id,
                aggregate_version,
                event_name,
                payload,
                occurred_at,
                correlation_id,
                causation_id,
                context,
                published_at,
                attempt_count,
                next_attempt_after,
                lease_owner,
                lease_until,
                last_error,
                dead_lettered_at
            ) VALUES
            "#,
        );

        {
            let mut separated = query_builder.separated(", ");
            for outbox in dead_lettered_outboxes {
                let outbox_id = outbox.id.value();
                let event = &outbox.event;
                let context_value = serde_json::to_value(&event.context)
                    .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;
                let last_error_value = Self::serialize_last_error(outbox)?;
                let dead_lettered_at_value = match outbox.lifecycle {
                    OutboxLifecycle::DeadLettered { dead_lettered_at } => {
                        DateTime::<Utc>::from(dead_lettered_at)
                    }
                    OutboxLifecycle::Active => Utc::now(),
                };

                separated
                    .push("(")
                    .push_bind(outbox_id)
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
                    .push_bind(context_value)
                    .push_bind(outbox.state.published_at().map(DateTime::<Utc>::from))
                    .push_bind(outbox.state.attempt_count().value())
                    .push_bind(outbox.state.next_attempt_after().map(DateTime::<Utc>::from))
                    .push_bind(outbox.state.lease_owner().map(ToString::to_string))
                    .push_bind(outbox.state.lease_until().map(DateTime::<Utc>::from))
                    .push_bind(last_error_value)
                    .push_bind(dead_lettered_at_value)
                    .push(")");
            }
        }

        let transaction = uow.transaction_mut();

        query_builder
            .build()
            .execute(transaction.as_mut())
            .await
            .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;

        Ok(())
    }

    async fn delete_outboxes(
        uow: &mut PgUnitOfWork,
        dead_lettered_outboxes: &[&EventOutbox],
    ) -> Result<(), OutboxWriterError> {
        let mut query_builder =
            QueryBuilder::<Postgres>::new("DELETE FROM event_outbox WHERE id IN (");

        {
            let mut separated = query_builder.separated(", ");
            for outbox in dead_lettered_outboxes {
                separated.push_bind(outbox.id.value());
            }
        }

        query_builder.push(")");

        let transaction = uow.transaction_mut();

        query_builder
            .build()
            .execute(transaction.as_mut())
            .await
            .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;

        Ok(())
    }

    async fn delete_dead_letters(
        uow: &mut PgUnitOfWork,
        active_outboxes: &[&EventOutbox],
    ) -> Result<(), OutboxWriterError> {
        if active_outboxes.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            DELETE FROM event_dead_letters
             WHERE event_outbox_id IN (
            "#,
        );

        {
            let mut separated = query_builder.separated(", ");
            for outbox in active_outboxes {
                separated.push_bind(outbox.id.value());
            }
        }

        query_builder.push(")");

        let transaction = uow.transaction_mut();

        query_builder
            .build()
            .execute(transaction.as_mut())
            .await
            .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;

        Ok(())
    }
}

impl Default for PgEventOutboxWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutboxWriter for PgEventOutboxWriter {
    type Uow = PgUnitOfWork;
    type Outbox = EventOutbox;

    async fn write_outbox(
        &self,
        uow: &mut Self::Uow,
        outboxes: &[EventOutbox],
    ) -> Result<(), OutboxWriterError> {
        if outboxes.is_empty() {
            return Ok(());
        }

        let mut active_outboxes: Vec<&EventOutbox> = Vec::new();
        let mut dead_lettered_outboxes: Vec<&EventOutbox> = Vec::new();
        for outbox in outboxes {
            if matches!(outbox.lifecycle, OutboxLifecycle::DeadLettered { .. }) {
                dead_lettered_outboxes.push(outbox);
            } else {
                active_outboxes.push(outbox);
            }
        }

        Self::upsert_outbox_rows(uow, &active_outboxes).await?;
        Self::delete_dead_letters(uow, &active_outboxes).await?;

        if !dead_lettered_outboxes.is_empty() {
            Self::insert_dead_letters(uow, &dead_lettered_outboxes).await?;
            Self::delete_outboxes(uow, &dead_lettered_outboxes).await?;
        }

        Ok(())
    }
}
