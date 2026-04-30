use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::request_context::CorrelationId;
use appletheia_application::saga::{
    SagaInstance, SagaInstanceId, SagaNameOwned, SagaState, SagaStatus,
};

use super::pg_saga_instance_row_error::PgSagaInstanceRowError;

#[derive(Debug, FromRow)]
pub struct PgSagaInstanceRow {
    pub id: Uuid,
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
        let saga_instance_id = SagaInstanceId::try_from(self.id)?;

        let (status, state) = match (self.succeeded_at, self.failed_at, self.state) {
            (Some(_), None, Some(state_json)) => {
                let state: S = serde_json::from_value(state_json)?;
                (SagaStatus::Succeeded, Some(state))
            }
            (Some(_), None, None) => (SagaStatus::Succeeded, None),
            (None, Some(_), Some(state_json)) => {
                let state: S = serde_json::from_value(state_json)?;
                (SagaStatus::Failed, Some(state))
            }
            (None, Some(_), None) => (SagaStatus::Failed, None),
            (None, None, state_json) => {
                let state = match state_json {
                    Some(value) => Some(serde_json::from_value(value)?),
                    None => None,
                };
                (SagaStatus::InProgress, state)
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

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use super::PgSagaInstanceRow;
    use appletheia_application::request_context::CorrelationId;
    use appletheia_application::saga::{SagaName, SagaNameOwned, SagaState, SagaStatus};

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct TestSagaState;

    impl SagaState for TestSagaState {}

    #[test]
    fn try_into_instance_allows_succeeded_without_state() {
        let row = PgSagaInstanceRow {
            id: Uuid::now_v7(),
            state: None,
            succeeded_at: Some(Utc::now()),
            failed_at: None,
        };

        let instance = row
            .try_into_instance::<TestSagaState>(
                SagaNameOwned::from(SagaName::new("test_saga")),
                CorrelationId::from(Uuid::now_v7()),
            )
            .expect("succeeded row without state should deserialize");

        assert_eq!(instance.status, SagaStatus::Succeeded);
        assert!(instance.state.is_none());
    }

    #[test]
    fn try_into_instance_allows_failed_without_state() {
        let row = PgSagaInstanceRow {
            id: Uuid::now_v7(),
            state: None,
            succeeded_at: None,
            failed_at: Some(Utc::now()),
        };

        let instance = row
            .try_into_instance::<TestSagaState>(
                SagaNameOwned::from(SagaName::new("test_saga")),
                CorrelationId::from(Uuid::now_v7()),
            )
            .expect("failed row without state should deserialize");

        assert_eq!(instance.status, SagaStatus::Failed);
        assert!(instance.state.is_none());
    }
}
