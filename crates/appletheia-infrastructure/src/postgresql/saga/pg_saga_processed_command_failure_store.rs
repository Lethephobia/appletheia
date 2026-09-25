use appletheia_application::command::CommandFailureId;
use appletheia_application::request_context::MessageId;
use appletheia_application::saga::{
    SagaInstanceId, SagaProcessedCommandFailureId, SagaProcessedCommandFailureStore,
    SagaProcessedCommandFailureStoreError,
};

use crate::postgresql::unit_of_work::PgUnitOfWork;

use super::pg_saga_processed_command_failure_row::PgSagaProcessedCommandFailureRow;

#[derive(Debug)]
pub struct PgSagaProcessedCommandFailureStore;

impl PgSagaProcessedCommandFailureStore {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgSagaProcessedCommandFailureStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SagaProcessedCommandFailureStore for PgSagaProcessedCommandFailureStore {
    type Uow = PgUnitOfWork;

    async fn mark_processed(
        &self,
        uow: &mut Self::Uow,
        saga_instance_id: SagaInstanceId,
        command_failure_id: CommandFailureId,
        command_message_id: MessageId,
    ) -> Result<bool, SagaProcessedCommandFailureStoreError> {
        let transaction = uow.transaction_mut();
        let id_value = SagaProcessedCommandFailureId::new().value();

        let row = sqlx::query_as::<_, PgSagaProcessedCommandFailureRow>(
            r#"
            INSERT INTO saga_processed_command_failures (
              id,
              saga_instance_id,
              command_failure_id,
              command_message_id
            ) VALUES (
              $1,
              $2,
              $3,
              $4
            )
            ON CONFLICT DO NOTHING
            RETURNING
              id,
              saga_instance_id,
              command_failure_id,
              command_message_id,
              processed_at
            "#,
        )
        .bind(id_value)
        .bind(saga_instance_id.value())
        .bind(command_failure_id.value())
        .bind(command_message_id.value())
        .fetch_optional(transaction.as_mut())
        .await
        .map_err(|source| SagaProcessedCommandFailureStoreError::Persistence(Box::new(source)))?;

        Ok(row.is_some())
    }
}
