use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::request_context::CorrelationId;
use appletheia_application::saga::{
    SagaInstance, SagaInstanceId, SagaNameOwned, SagaState, SagaStatus,
};

use super::pg_saga_instance_row_error::PgSagaInstanceRowError;

#[derive(Debug, FromRow)]
pub(super) struct PgSagaInstanceRow {
    pub saga_instance_id: Uuid,
    pub state: Option<serde_json::Value>,
    pub succeeded_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
}

impl PgSagaInstanceRow {
    pub fn try_into_instance<S: SagaState>(
        self,
        saga_name: SagaNameOwned,
        correlation_id: CorrelationId,
    ) -> Result<SagaInstance<S>, PgSagaInstanceRowError> {
        let saga_instance_id = SagaInstanceId::try_from(self.saga_instance_id)?;

        let (status, state) = match (self.succeeded_at, self.failed_at, self.state) {
            (Some(_), None, Some(state_json)) => {
                let state: S = serde_json::from_value(state_json)?;
                (SagaStatus::Succeeded, Some(state))
            }
            (None, Some(_), Some(state_json)) => {
                let state: S = serde_json::from_value(state_json)?;
                (SagaStatus::Failed, Some(state))
            }
            (None, None, state_json) => {
                let state = match state_json {
                    Some(value) => Some(serde_json::from_value(value)?),
                    None => None,
                };
                (SagaStatus::InProgress, state)
            }
            (Some(_), _, None) => {
                return Err(PgSagaInstanceRowError::InvalidPersistedInstance {
                    message: "succeeded instance must have non-null state",
                });
            }
            (None, Some(_), None) => {
                return Err(PgSagaInstanceRowError::InvalidPersistedInstance {
                    message: "failed instance must have non-null state",
                });
            }
            (Some(_), Some(_), _) => {
                return Err(PgSagaInstanceRowError::InvalidPersistedInstance {
                    message: "instance cannot be both succeeded and failed",
                });
            }
        };

        Ok(SagaInstance {
            saga_instance_id,
            saga_name,
            correlation_id,
            status,
            state,
            uncommitted_commands: Vec::new(),
        })
    }
}
