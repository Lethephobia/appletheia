use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

use crate::postgresql::unit_of_work::PgUnitOfWork;
use appletheia_application::outbox::OrderingKey;
use appletheia_application::outbox::command::{
    CommandEnvelope, CommandOutboxEnqueueError, CommandOutboxEnqueuer,
};

pub struct PgCommandOutboxEnqueuer;

impl PgCommandOutboxEnqueuer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgCommandOutboxEnqueuer {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandOutboxEnqueuer for PgCommandOutboxEnqueuer {
    type Uow = PgUnitOfWork;

    async fn enqueue_commands(
        &self,
        uow: &mut Self::Uow,
        ordering_key: &OrderingKey,
        commands: &[CommandEnvelope],
    ) -> Result<(), CommandOutboxEnqueueError> {
        if commands.is_empty() {
            return Ok(());
        }

        let transaction = uow.transaction_mut();

        let ordering_key_value = ordering_key.as_str();

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
                let command_name_value = command.command_name.value();
                let payload_value = command.command.value().clone();
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
                    .push_bind(ordering_key_value)
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
