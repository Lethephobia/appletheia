use std::marker::PhantomData;

use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

use appletheia_application::command::CommandName;
use appletheia_application::outbox::OrderingKey;
use appletheia_application::outbox::command::{
    CommandEnvelope, CommandOutboxEnqueueError, CommandOutboxEnqueuer,
};
use appletheia_application::unit_of_work::UnitOfWorkError;

use crate::postgresql::unit_of_work::PgUnitOfWork;

pub struct PgCommandOutboxEnqueuer<CN> {
    _marker: PhantomData<CN>,
}

impl<CN> PgCommandOutboxEnqueuer<CN> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    fn map_uow_error(error: UnitOfWorkError) -> CommandOutboxEnqueueError {
        match error {
            UnitOfWorkError::NotInTransaction => CommandOutboxEnqueueError::NotInTransaction,
            other => CommandOutboxEnqueueError::Persistence(Box::new(other)),
        }
    }
}

impl<CN> Default for PgCommandOutboxEnqueuer<CN> {
    fn default() -> Self {
        Self::new()
    }
}

impl<CN: CommandName> CommandOutboxEnqueuer for PgCommandOutboxEnqueuer<CN> {
    type Uow = PgUnitOfWork;
    type CommandName = CN;

    async fn enqueue_commands(
        &self,
        uow: &mut Self::Uow,
        ordering_key: &OrderingKey,
        commands: &[CommandEnvelope<CN>],
    ) -> Result<(), CommandOutboxEnqueueError> {
        if commands.is_empty() {
            return Ok(());
        }

        let transaction = uow.transaction_mut().map_err(Self::map_uow_error)?;

        let ordering_key_value = ordering_key.to_string();

        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO command_outbox (
              id,
              message_id,
              command_name,
              payload,
              correlation_id,
              causation_id,
              ordering_key
            ) VALUES
            "#,
        );

        {
            let mut separated = query_builder.separated(", ");
            for command in commands {
                let id_value = Uuid::now_v7();
                let message_id_value = command.message_id.value();
                let command_name_value = command.command_name.to_string();
                let payload_value = command.payload.value().clone();
                let correlation_id_value = command.correlation_id.0;
                let causation_id_value = command.causation_id.value();

                separated
                    .push("(")
                    .push_bind(id_value)
                    .push_bind(message_id_value)
                    .push_bind(command_name_value)
                    .push_bind(payload_value)
                    .push_bind(correlation_id_value)
                    .push_bind(causation_id_value)
                    .push_bind(&ordering_key_value)
                    .push(")");
            }
        }

        query_builder
            .build()
            .execute(transaction.as_mut())
            .await
            .map_err(|source| CommandOutboxEnqueueError::Persistence(Box::new(source)))?;

        Ok(())
    }
}
