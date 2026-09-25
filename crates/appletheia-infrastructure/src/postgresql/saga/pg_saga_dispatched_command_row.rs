use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::command::CommandNameOwned;
use appletheia_application::request_context::MessageId;
use appletheia_application::saga::{SagaDispatchedCommand, SagaStep};

use super::pg_saga_dispatched_command_row_error::PgSagaDispatchedCommandRowError;

#[derive(Debug, FromRow)]
pub struct PgSagaDispatchedCommandRow {
    pub message_id: Uuid,
    pub command_name: String,
    pub step: serde_json::Value,
}

impl PgSagaDispatchedCommandRow {
    pub fn try_into_dispatched_command<S: SagaStep>(
        self,
    ) -> Result<SagaDispatchedCommand<S>, PgSagaDispatchedCommandRowError> {
        Ok(SagaDispatchedCommand {
            message_id: MessageId::from(self.message_id),
            command_name: CommandNameOwned::new(self.command_name)?,
            step: serde_json::from_value(self.step)?,
        })
    }
}
