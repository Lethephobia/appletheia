use std::error::Error;

use serde::{Serialize, de::DeserializeOwned};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::request_context::CorrelationId;
use appletheia_application::saga::{SagaNameOwned, SagaRun, SagaRunId};

#[derive(Debug, FromRow)]
pub struct PgSagaRunRow {
    pub id: Uuid,
    pub context: serde_json::Value,
}

impl PgSagaRunRow {
    pub fn try_into_run<C: Serialize + DeserializeOwned + Send + Sync + 'static>(
        self,
        saga_name: SagaNameOwned,
        correlation_id: CorrelationId,
    ) -> Result<SagaRun<C>, Box<dyn Error + Send + Sync>> {
        let saga_run_id = SagaRunId::try_from(self.id)?;

        let context = serde_json::from_value(self.context)?;

        Ok(SagaRun {
            saga_run_id,
            saga_name,
            correlation_id,
            context,
        })
    }
}
