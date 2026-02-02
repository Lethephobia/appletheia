use appletheia_application::command::CommandFailureReport;
use appletheia_application::command::{
    CommandHash, CommandName, IdempotencyBeginResult, IdempotencyError, IdempotencyOutput,
    IdempotencyService, IdempotencyState,
};
use appletheia_application::request_context::MessageId;

use crate::postgresql::unit_of_work::PgUnitOfWork;

use super::pg_idempotency_row::IdempotencyRow;

#[derive(Debug)]
pub struct PgIdempotencyService;

impl PgIdempotencyService {
    pub fn new() -> Self {
        Self
    }

    fn is_in_progress_lock_error(source: &sqlx::Error) -> bool {
        let Some(db_error) = source.as_database_error() else {
            return false;
        };

        // - 55P03: lock_not_available (e.g. NOWAIT) / can appear depending on settings
        // - 57014: query_canceled (e.g. statement_timeout/lock_timeout)
        // For our use case, treat these as "someone else is already processing it".
        matches!(db_error.code().as_deref(), Some("55P03" | "57014"))
    }
}

impl Default for PgIdempotencyService {
    fn default() -> Self {
        Self::new()
    }
}

impl IdempotencyService for PgIdempotencyService {
    type Uow = PgUnitOfWork;

    async fn begin(
        &self,
        uow: &mut Self::Uow,
        message_id: MessageId,
        command_name: CommandName,
        command_hash: &CommandHash,
    ) -> Result<IdempotencyBeginResult, IdempotencyError> {
        let transaction = uow.transaction_mut();

        let message_id_value = message_id.value();
        let command_name_value = command_name.to_string();
        let command_hash_value = command_hash.as_str();

        let insert_result = sqlx::query(
            r#"
            INSERT INTO idempotency (
              message_id,
              command_name,
              command_hash
            ) VALUES (
              $1,
              $2,
              $3
            )
            ON CONFLICT (message_id) DO NOTHING
            "#,
        )
        .bind(message_id_value)
        .bind(&command_name_value)
        .bind(command_hash_value)
        .execute(transaction.as_mut())
        .await;

        match insert_result {
            Ok(done) if done.rows_affected() == 1 => return Ok(IdempotencyBeginResult::New),
            Ok(_) => {}
            Err(source) if Self::is_in_progress_lock_error(&source) => {
                return Ok(IdempotencyBeginResult::InProgress);
            }
            Err(source) => return Err(IdempotencyError::Persistence(Box::new(source))),
        }

        let row: IdempotencyRow = sqlx::query_as(
            r#"
            SELECT
              command_name,
              command_hash,
              completed_at,
              output,
              error
            FROM idempotency
            WHERE message_id = $1
            "#,
        )
        .bind(message_id_value)
        .fetch_one(transaction.as_mut())
        .await
        .map_err(|source| IdempotencyError::Persistence(Box::new(source)))?;

        if row.command_name != command_name_value || row.command_hash != command_hash_value {
            return Err(IdempotencyError::Conflict { message_id });
        }

        match row.completed_at {
            None => Ok(IdempotencyBeginResult::InProgress),
            Some(_) => match (row.output, row.error) {
                (Some(output), None) => Ok(IdempotencyBeginResult::Existing {
                    state: IdempotencyState::Succeeded {
                        output: IdempotencyOutput::from(output),
                    },
                }),
                (None, Some(error)) => {
                    let error = serde_json::from_value(error)
                        .map_err(|source| IdempotencyError::Persistence(Box::new(source)))?;
                    Ok(IdempotencyBeginResult::Existing {
                        state: IdempotencyState::Failed { error },
                    })
                }
                _ => Err(IdempotencyError::InvalidStateTransition),
            },
        }
    }

    async fn complete_success(
        &self,
        uow: &mut Self::Uow,
        message_id: MessageId,
        output: IdempotencyOutput,
    ) -> Result<(), IdempotencyError> {
        let transaction = uow.transaction_mut();

        let message_id_value = message_id.value();

        let updated = sqlx::query(
            r#"
            UPDATE idempotency
               SET completed_at = now(),
                   output = $2,
                   error = NULL
             WHERE message_id = $1
               AND completed_at IS NULL
            "#,
        )
        .bind(message_id_value)
        .bind(serde_json::Value::from(output))
        .execute(transaction.as_mut())
        .await
        .map_err(|source| IdempotencyError::Persistence(Box::new(source)))?;

        if updated.rows_affected() != 1 {
            return Err(IdempotencyError::InvalidStateTransition);
        }

        Ok(())
    }

    async fn complete_failure(
        &self,
        uow: &mut Self::Uow,
        message_id: MessageId,
        error: CommandFailureReport,
    ) -> Result<(), IdempotencyError> {
        let transaction = uow.transaction_mut();

        let message_id_value = message_id.value();

        let error_json = serde_json::to_value(error)
            .map_err(|source| IdempotencyError::Persistence(Box::new(source)))?;

        let updated = sqlx::query(
            r#"
            UPDATE idempotency
               SET completed_at = now(),
                   output = NULL,
                   error = $2
             WHERE message_id = $1
               AND completed_at IS NULL
            "#,
        )
        .bind(message_id_value)
        .bind(error_json)
        .execute(transaction.as_mut())
        .await
        .map_err(|source| IdempotencyError::Persistence(Box::new(source)))?;

        if updated.rows_affected() != 1 {
            return Err(IdempotencyError::InvalidStateTransition);
        }

        Ok(())
    }
}
