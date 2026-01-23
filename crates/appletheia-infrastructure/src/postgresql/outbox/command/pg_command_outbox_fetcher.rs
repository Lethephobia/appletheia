use std::marker::PhantomData;

use chrono::Utc;
use sqlx::Postgres;

use appletheia_application::command::CommandName;
use appletheia_application::outbox::{
    OutboxBatchSize, OutboxFetcher, OutboxFetcherError, command::CommandOutbox,
};
use appletheia_application::unit_of_work::UnitOfWorkError;

use crate::postgresql::unit_of_work::PgUnitOfWork;

use super::{PgCommandOutboxRow, PgCommandOutboxRowError};

pub struct PgCommandOutboxFetcher<CN> {
    _marker: PhantomData<CN>,
}

impl<CN> PgCommandOutboxFetcher<CN> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<CN> Default for PgCommandOutboxFetcher<CN> {
    fn default() -> Self {
        Self::new()
    }
}

impl<CN: CommandName> OutboxFetcher for PgCommandOutboxFetcher<CN> {
    type Uow = PgUnitOfWork;
    type Outbox = CommandOutbox<CN>;

    async fn fetch(
        &self,
        uow: &mut Self::Uow,
        limit: OutboxBatchSize,
    ) -> Result<Vec<CommandOutbox<CN>>, OutboxFetcherError> {
        let now = Utc::now();

        let transaction = uow.transaction_mut().map_err(|e| match e {
            UnitOfWorkError::NotInTransaction => OutboxFetcherError::NotInTransaction,
            other => OutboxFetcherError::Persistence(Box::new(other)),
        })?;

        let outbox_rows = sqlx::query_as::<Postgres, PgCommandOutboxRow>(
            r#"
            SELECT
                id,
                command_sequence,
                message_id,
                command_name,
                payload,
                correlation_id,
                causation_id,
                ordering_key,
                published_at,
                attempt_count,
                next_attempt_after,
                lease_owner,
                lease_until,
                last_error
            FROM command_outbox
            WHERE published_at IS NULL
              AND next_attempt_after <= $1
              AND (lease_owner IS NULL OR lease_until <= $1)
            ORDER BY next_attempt_after ASC, command_sequence ASC
            LIMIT $2
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .bind(now)
        .bind(limit.as_i64())
        .fetch_all(transaction.as_mut())
        .await
        .map_err(|e| OutboxFetcherError::Persistence(Box::new(e)))?;

        if outbox_rows.is_empty() {
            return Ok(Vec::new());
        }

        let outboxes = outbox_rows
            .into_iter()
            .map(PgCommandOutboxRow::try_into_outbox::<CN>)
            .collect::<Result<Vec<CommandOutbox<CN>>, PgCommandOutboxRowError>>()
            .map_err(|e| OutboxFetcherError::MappingFailed(Box::new(e)))?;

        Ok(outboxes)
    }
}
