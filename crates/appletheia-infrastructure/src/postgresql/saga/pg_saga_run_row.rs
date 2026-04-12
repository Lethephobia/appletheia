use std::error::Error;

use serde::{Serialize, de::DeserializeOwned};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::request_context::{CorrelationId, MessageId};
use appletheia_application::saga::{SagaNameOwned, SagaRun, SagaRunId};
use appletheia_domain::EventId;

#[derive(Debug, FromRow)]
pub struct PgSagaRunRow {
    pub id: Uuid,
    pub trigger_event_id: Uuid,
    pub dispatched_command_message_id: Uuid,
    pub context: serde_json::Value,
}

impl PgSagaRunRow {
    pub fn try_into_run<C: Serialize + DeserializeOwned + Send + Sync + 'static>(
        self,
        saga_name: SagaNameOwned,
        correlation_id: CorrelationId,
    ) -> Result<SagaRun<C>, Box<dyn Error + Send + Sync>> {
        let saga_run_id = SagaRunId::try_from(self.id)?;
        let trigger_event_id = EventId::try_from(self.trigger_event_id)?;
        let dispatched_command_message_id = MessageId::from(self.dispatched_command_message_id);

        let context = serde_json::from_value(self.context)?;

        Ok(SagaRun {
            saga_run_id,
            saga_name,
            correlation_id,
            trigger_event_id,
            dispatched_command_message_id,
            context,
        })
    }
}
