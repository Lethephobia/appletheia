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

    async fn update_outbox_row(
        uow: &mut PgUnitOfWork,
        outbox: &CommandOutbox,
    ) -> Result<(), OutboxWriterError> {
        let outbox_id = outbox.id.value();

        let published_at_value = outbox.state.published_at().map(DateTime::<Utc>::from);
        let attempt_count_value = outbox.state.attempt_count().value();
        let next_attempt_after_value = outbox.state.next_attempt_after().map(DateTime::<Utc>::from);
        let lease_owner_value = outbox.state.lease_owner().map(ToString::to_string);
        let lease_until_value = outbox.state.lease_until().map(DateTime::<Utc>::from);

        let last_error_value = Self::serialize_last_error(outbox)?;

        let transaction = uow.transaction_mut();

        sqlx::query(
            r#"
            UPDATE command_outbox
               SET published_at = $2,
                   attempt_count = $3,
                   next_attempt_after = COALESCE($4, next_attempt_after),
                   lease_owner = $5,
                   lease_until = $6,
                   last_error = $7
             WHERE id = $1
            "#,
        )
        .bind(outbox_id)
        .bind(published_at_value)
        .bind(attempt_count_value)
        .bind(next_attempt_after_value)
        .bind(lease_owner_value)
        .bind(lease_until_value)
        .bind(last_error_value)
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
                let outbox_id = outbox.id.value();

                let command = &outbox.command;
                let command_sequence_value = outbox.sequence;
                let message_id_value = command.message_id.value();
                let command_name_value = command.command_name.value();
                let payload_value = command.command.value().clone();
                let correlation_id_value = command.correlation_id.0;
                let causation_id_value = command.causation_id.value();

                let published_at_value = outbox.state.published_at().map(DateTime::<Utc>::from);
                let attempt_count_value = outbox.state.attempt_count().value();
                let next_attempt_after_value =
                    outbox.state.next_attempt_after().map(DateTime::<Utc>::from);
                let lease_owner_value = outbox.state.lease_owner().map(ToString::to_string);
                let lease_until_value = outbox.state.lease_until().map(DateTime::<Utc>::from);

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
                    .push_bind(command_sequence_value)
                    .push_bind(message_id_value)
                    .push_bind(command_name_value)
                    .push_bind(payload_value)
                    .push_bind(correlation_id_value)
                    .push_bind(causation_id_value)
                    .push_bind(published_at_value)
                    .push_bind(attempt_count_value)
                    .push_bind(next_attempt_after_value)
                    .push_bind(lease_owner_value)
                    .push_bind(lease_until_value)
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
                let outbox_id = outbox.id.value();
                separated.push_bind(outbox_id);
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

        let mut dead_lettered_outboxes: Vec<&CommandOutbox> = Vec::new();
        for outbox in outboxes {
            if matches!(outbox.lifecycle, OutboxLifecycle::DeadLettered { .. }) {
                dead_lettered_outboxes.push(outbox);
            }
        }

        for outbox in outboxes {
            Self::update_outbox_row(uow, outbox).await?;
        }

        if !dead_lettered_outboxes.is_empty() {
            Self::insert_dead_letters(uow, &dead_lettered_outboxes).await?;
            Self::delete_outboxes(uow, &dead_lettered_outboxes).await?;
        }

        Ok(())
    }
}
