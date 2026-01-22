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
    pub last_error: Option<serde_json::Value>,
}

impl PgSagaInstanceRow {
    pub fn try_into_instance<S: SagaState>(
        self,
        saga_name: SagaNameOwned,
        correlation_id: CorrelationId,
    ) -> Result<SagaInstance<S>, PgSagaInstanceRowError> {
        let saga_instance_id = SagaInstanceId::try_from(self.saga_instance_id)?;

        let status = match (
            self.succeeded_at,
            self.failed_at,
            self.state,
            self.last_error,
        ) {
            (Some(_), None, Some(state_json), _) => {
                let state: S = serde_json::from_value(state_json)?;
                SagaStatus::Succeeded { state }
            }
            (None, Some(_), Some(state_json), Some(error)) => {
                let state: S = serde_json::from_value(state_json)?;
                SagaStatus::Failed { state, error }
            }
            (None, None, state_json, _) => {
                let state = match state_json {
                    Some(value) => Some(serde_json::from_value(value)?),
                    None => None,
                };
                SagaStatus::InProgress { state }
            }
            (Some(_), _, None, _) => {
                return Err(PgSagaInstanceRowError::InvalidPersistedInstance {
                    message: "succeeded instance must have non-null state",
                });
            }
            (None, Some(_), None, _) => {
                return Err(PgSagaInstanceRowError::InvalidPersistedInstance {
                    message: "failed instance must have non-null state",
                });
            }
            (None, Some(_), _, None) => {
                return Err(PgSagaInstanceRowError::InvalidPersistedInstance {
                    message: "failed instance must have non-null last_error",
                });
            }
            (Some(_), Some(_), _, _) => {
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
        })
    }
}
