use crate::postgresql::saga::pg_saga_run_row::PgSagaRunRow;
use crate::postgresql::unit_of_work::PgUnitOfWork;
use appletheia_application::request_context::MessageId;
use appletheia_application::saga::{SagaNameOwned, SagaRun, SagaRunStore, SagaRunStoreError};
use appletheia_domain::EventId;
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug)]
pub struct PgSagaRunStore;

impl PgSagaRunStore {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgSagaRunStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SagaRunStore for PgSagaRunStore {
    type Uow = PgUnitOfWork;

    async fn read_by_trigger_event<C: Serialize + DeserializeOwned + Send + Sync + 'static>(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaNameOwned,
        trigger_event_id: EventId,
    ) -> Result<Option<SagaRun<C>>, SagaRunStoreError> {
        let transaction = uow.transaction_mut();

        let saga_name_value = saga_name.value();
        let trigger_event_id_value = trigger_event_id.value();

        let row = sqlx::query_as::<_, PgSagaRunRow>(
            r#"
            SELECT
              id,
              trigger_event_id,
              dispatched_command_message_id,
              context
            FROM saga_runs
            WHERE saga_name = $1
              AND trigger_event_id = $2
            FOR UPDATE
            "#,
        )
        .bind(saga_name_value)
        .bind(trigger_event_id_value)
        .fetch_optional(transaction.as_mut())
        .await
        .map_err(|source| SagaRunStoreError::Persistence(Box::new(source)))?;

        row.map(|row| {
            row.try_into_run::<C>(saga_name)
                .map_err(SagaRunStoreError::MappingFailed)
        })
        .transpose()
    }

    async fn read_by_dispatched_command_message<
        C: Serialize + DeserializeOwned + Send + Sync + 'static,
    >(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaNameOwned,
        dispatched_command_message_id: MessageId,
    ) -> Result<Option<SagaRun<C>>, SagaRunStoreError> {
        let transaction = uow.transaction_mut();

        let saga_name_value = saga_name.value();
        let dispatched_command_message_id_value = dispatched_command_message_id.value();

        let row = sqlx::query_as::<_, PgSagaRunRow>(
            r#"
            SELECT
              id,
              trigger_event_id,
              dispatched_command_message_id,
              context
            FROM saga_runs
            WHERE saga_name = $1
              AND dispatched_command_message_id = $2
            FOR UPDATE
            "#,
        )
        .bind(saga_name_value)
        .bind(dispatched_command_message_id_value)
        .fetch_optional(transaction.as_mut())
        .await
        .map_err(|source| SagaRunStoreError::Persistence(Box::new(source)))?;

        row.map(|row| {
            row.try_into_run::<C>(saga_name)
                .map_err(SagaRunStoreError::MappingFailed)
        })
        .transpose()
    }

    async fn write<C: Serialize + DeserializeOwned + Send + Sync + 'static>(
        &self,
        uow: &mut Self::Uow,
        run: &SagaRun<C>,
    ) -> Result<(), SagaRunStoreError> {
        let transaction = uow.transaction_mut();

        let saga_run_id_value = run.saga_run_id.value();

        let context_json =
            serde_json::to_value(&run.context).map_err(SagaRunStoreError::ContextSerialize)?;

        let updated = sqlx::query(
            r#"
            INSERT INTO saga_runs (
              id,
              saga_name,
              trigger_event_id,
              dispatched_command_message_id,
              context
            ) VALUES (
              $1,
              $2,
              $3,
              $4,
              $5
            )
            "#,
        )
        .bind(saga_run_id_value)
        .bind(run.saga_name.value())
        .bind(run.trigger_event_id.value())
        .bind(run.dispatched_command_message_id.value())
        .bind(&context_json)
        .execute(transaction.as_mut())
        .await
        .map_err(|source| SagaRunStoreError::Persistence(Box::new(source)))?;

        if updated.rows_affected() != 1 {
            return Err(SagaRunStoreError::Persistence(Box::new(
                std::io::Error::other("failed to write saga run row"),
            )));
        }

        Ok(())
    }
}
