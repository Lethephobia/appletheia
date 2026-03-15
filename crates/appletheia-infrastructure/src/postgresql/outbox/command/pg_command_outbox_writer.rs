use chrono::{DateTime, Utc};
use sqlx::{Postgres, QueryBuilder};

use appletheia_application::messaging::PublishDispatchError;
use appletheia_application::outbox::{
    OutboxLifecycle, OutboxWriter, OutboxWriterError, command::CommandOutbox,
};

use crate::postgresql::unit_of_work::PgUnitOfWork;

pub struct PgCommandOutboxWriter;

impl PgCommandOutboxWriter {
    pub fn new() -> Self {
        Self
    }

    fn serialize_last_error(
        outbox: &CommandOutbox,
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
        outboxes: &[&CommandOutbox],
    ) -> Result<(), OutboxWriterError> {
        if outboxes.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO command_outbox (
                id,
                command_sequence,
                message_id,
                command_name,
                payload,
                correlation_id,
                causation_id,
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
                let command = &outbox.command;
                let last_error_value = Self::serialize_last_error(outbox)?;
                let next_attempt_after_value = outbox
                    .state
                    .next_attempt_after()
                    .unwrap_or_default()
                    .value();

                separated
                    .push("(")
                    .push_bind(outbox.id.value())
                    .push_bind(outbox.sequence)
                    .push_bind(command.message_id.value())
                    .push_bind(command.command_name.value())
                    .push_bind(command.command.value().clone())
                    .push_bind(command.correlation_id.value())
                    .push_bind(command.causation_id.value())
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
        dead_lettered_outboxes: &[&CommandOutbox],
    ) -> Result<(), OutboxWriterError> {
        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO command_dead_letters (
                command_outbox_id,
                command_sequence,
                message_id,
                command_name,
                payload,
                correlation_id,
                causation_id,
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
                let command = &outbox.command;
                let last_error_value = Self::serialize_last_error(outbox)?;
                let dead_lettered_at_value = match outbox.lifecycle {
                    OutboxLifecycle::DeadLettered { dead_lettered_at } => {
                        DateTime::<Utc>::from(dead_lettered_at)
                    }
                    OutboxLifecycle::Active => Utc::now(),
                };

                separated
                    .push("(")
                    .push_bind(outbox.id.value())
                    .push_bind(outbox.sequence)
                    .push_bind(command.message_id.value())
                    .push_bind(command.command_name.value())
                    .push_bind(command.command.value().clone())
                    .push_bind(command.correlation_id.value())
                    .push_bind(command.causation_id.value())
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
        dead_lettered_outboxes: &[&CommandOutbox],
    ) -> Result<(), OutboxWriterError> {
        let mut query_builder =
            QueryBuilder::<Postgres>::new("DELETE FROM command_outbox WHERE id IN (");

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
        active_outboxes: &[&CommandOutbox],
    ) -> Result<(), OutboxWriterError> {
        if active_outboxes.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            DELETE FROM command_dead_letters
             WHERE command_outbox_id IN (
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

impl Default for PgCommandOutboxWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutboxWriter for PgCommandOutboxWriter {
    type Uow = PgUnitOfWork;
    type Outbox = CommandOutbox;

    async fn write_outbox(
        &self,
        uow: &mut Self::Uow,
        outboxes: &[CommandOutbox],
    ) -> Result<(), OutboxWriterError> {
        if outboxes.is_empty() {
            return Ok(());
        }

        let mut active_outboxes: Vec<&CommandOutbox> = Vec::new();
        let mut dead_lettered_outboxes: Vec<&CommandOutbox> = Vec::new();
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
