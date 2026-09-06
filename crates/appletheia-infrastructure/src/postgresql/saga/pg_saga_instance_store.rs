use crate::postgresql::saga::pg_saga_dispatched_command_row::PgSagaDispatchedCommandRow;
use crate::postgresql::saga::pg_saga_instance_row::PgSagaInstanceRow;
use crate::postgresql::unit_of_work::PgUnitOfWork;
use appletheia_application::request_context::{CorrelationId, MessageId};
use appletheia_application::saga::{
    SagaDispatchedCommand, SagaInstance, SagaInstanceStore, SagaInstanceStoreError, SagaNameOwned,
    SagaState, SagaStatus, SagaStep,
};

#[derive(Debug)]
pub struct PgSagaInstanceStore;

impl PgSagaInstanceStore {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgSagaInstanceStore {
    fn default() -> Self {
        Self::new()
    }
}

impl PgSagaInstanceStore {
    async fn read_dispatched_commands<S: SagaStep>(
        uow: &mut PgUnitOfWork,
        saga_instance_id: uuid::Uuid,
    ) -> Result<Vec<SagaDispatchedCommand<S>>, SagaInstanceStoreError> {
        let transaction = uow.transaction_mut();

        let rows = sqlx::query_as::<_, PgSagaDispatchedCommandRow>(
            r#"
            SELECT message_id, command_name, step
            FROM saga_dispatched_commands
            WHERE saga_instance_id = $1
            ORDER BY message_id ASC
            "#,
        )
        .bind(saga_instance_id)
        .fetch_all(transaction.as_mut())
        .await
        .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))?;

        rows.into_iter()
            .map(PgSagaDispatchedCommandRow::try_into_dispatched_command::<S>)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))
    }
}

impl SagaInstanceStore for PgSagaInstanceStore {
    type Uow = PgUnitOfWork;

    async fn find_by_correlation_id<S: SagaState, T: SagaStep>(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaNameOwned,
        correlation_id: CorrelationId,
    ) -> Result<Option<SagaInstance<S, T>>, SagaInstanceStoreError> {
        let transaction = uow.transaction_mut();

        let saga_name_value = saga_name.value();
        let correlation_id_value = correlation_id.value();

        let row = sqlx::query_as::<_, PgSagaInstanceRow>(
            r#"
            SELECT
              id,
              correlation_id,
              start_event_id,
              state,
              completed_at
            FROM saga_instances
            WHERE saga_name = $1
              AND correlation_id = $2
            FOR UPDATE
            "#,
        )
        .bind(saga_name_value)
        .bind(correlation_id_value)
        .fetch_optional(transaction.as_mut())
        .await
        .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))?;

        let Some(row) = row else {
            return Ok(None);
        };

        let dispatched_commands = Self::read_dispatched_commands::<T>(uow, row.id).await?;

        row.try_into_instance::<S, T>(saga_name, correlation_id, dispatched_commands)
            .map(Some)
            .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))
    }

    async fn find_by_dispatched_command_message_id<S: SagaState, T: SagaStep>(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaNameOwned,
        dispatched_command_message_id: MessageId,
    ) -> Result<Option<SagaInstance<S, T>>, SagaInstanceStoreError> {
        let transaction = uow.transaction_mut();

        let row = sqlx::query_as::<_, PgSagaInstanceRow>(
            r#"
            SELECT
              si.id,
              si.correlation_id,
              si.start_event_id,
              si.state,
              si.completed_at
            FROM saga_instances si
            JOIN saga_dispatched_commands sdc
              ON sdc.saga_instance_id = si.id
            WHERE si.saga_name = $1
              AND sdc.message_id = $2
            FOR UPDATE OF si
            "#,
        )
        .bind(saga_name.value())
        .bind(dispatched_command_message_id.value())
        .fetch_optional(transaction.as_mut())
        .await
        .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))?;

        let Some(row) = row else {
            return Ok(None);
        };

        let dispatched_commands = Self::read_dispatched_commands::<T>(uow, row.id).await?;
        let correlation_id = CorrelationId::from(row.correlation_id);

        row.try_into_instance::<S, T>(saga_name, correlation_id, dispatched_commands)
            .map(Some)
            .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))
    }

    async fn save<S: SagaState, T: SagaStep>(
        &self,
        uow: &mut Self::Uow,
        instance: &SagaInstance<S, T>,
    ) -> Result<(), SagaInstanceStoreError> {
        let transaction = uow.transaction_mut();

        let saga_instance_id_value = instance.saga_instance_id.value();

        let state_json = match instance.state.as_ref() {
            Some(state) => {
                Some(serde_json::to_value(state).map_err(SagaInstanceStoreError::StateSerialize)?)
            }
            None => None,
        };

        let completed = matches!(instance.status, SagaStatus::Completed);

        let persisted_saga_instance_id = sqlx::query_scalar::<_, uuid::Uuid>(
            r#"
            INSERT INTO saga_instances (
              id,
              saga_name,
              correlation_id,
              start_event_id,
              state,
              completed_at
            ) VALUES (
              $1,
              $2,
              $3,
              $4,
              $5,
              CASE WHEN $6 THEN now() ELSE NULL END
            )
            ON CONFLICT (saga_name, correlation_id) DO UPDATE SET
              state = EXCLUDED.state,
              completed_at = CASE WHEN $6 THEN COALESCE(saga_instances.completed_at, now()) ELSE NULL END
            RETURNING id
            "#,
        )
        .bind(saga_instance_id_value)
        .bind(instance.saga_name.value())
        .bind(instance.correlation_id.value())
        .bind(instance.start_event_id.value())
        .bind(state_json)
        .bind(completed)
        .fetch_one(transaction.as_mut())
        .await
        .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))?;

        for command in &instance.uncommitted_commands {
            let origin = command
                .saga_origin
                .as_ref()
                .ok_or(SagaInstanceStoreError::MissingCommandOrigin)?;
            if origin.saga_name != instance.saga_name
                || origin.saga_instance_id != instance.saga_instance_id
            {
                return Err(SagaInstanceStoreError::CommandOriginMismatch);
            }
            sqlx::query(
                r#"
                INSERT INTO saga_dispatched_commands (
                  saga_instance_id,
                  message_id,
                  command_name,
                  step
                ) VALUES (
                  $1,
                  $2,
                  $3,
                  $4
                )
                ON CONFLICT DO NOTHING
                "#,
            )
            .bind(persisted_saga_instance_id)
            .bind(command.message_id.value())
            .bind(command.command_name.value())
            .bind(origin.step.value().clone())
            .execute(transaction.as_mut())
            .await
            .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))?;
        }

        Ok(())
    }
}
