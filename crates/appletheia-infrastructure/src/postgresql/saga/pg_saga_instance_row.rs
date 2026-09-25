use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::request_context::CorrelationId;
use appletheia_application::saga::{
    SagaDispatchedCommand, SagaInstance, SagaInstanceId, SagaNameOwned, SagaState, SagaStep,
};
use appletheia_domain::EventId;

use super::pg_saga_instance_row_error::PgSagaInstanceRowError;

#[derive(Debug, FromRow)]
pub struct PgSagaInstanceRow {
    pub id: Uuid,
    pub correlation_id: Uuid,
    pub start_event_id: Uuid,
    pub state: Option<serde_json::Value>,
}

impl PgSagaInstanceRow {
    pub fn try_into_instance<S: SagaState, T: SagaStep>(
        self,
        saga_name: SagaNameOwned,
        correlation_id: CorrelationId,
        dispatched_commands: Vec<SagaDispatchedCommand<T>>,
    ) -> Result<SagaInstance<S, T>, PgSagaInstanceRowError> {
        let saga_instance_id = SagaInstanceId::try_from(self.id)?;
        let start_event_id = EventId::try_from(self.start_event_id)?;

        let state = self.state.map(serde_json::from_value).transpose()?;

        Ok(SagaInstance {
            saga_instance_id,
            saga_name,
            correlation_id,
            start_event_id,
            state,
            dispatched_commands,
            uncommitted_commands: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use super::PgSagaInstanceRow;
    use appletheia_application::request_context::CorrelationId;
    use appletheia_application::saga::{SagaName, SagaNameOwned, SagaState, SagaStep};

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct TestSagaState;

    impl SagaState for TestSagaState {}

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct TestSagaStep;

    impl SagaStep for TestSagaStep {}

    #[test]
    fn try_into_instance_allows_absent_state() {
        let row = PgSagaInstanceRow {
            id: Uuid::now_v7(),
            correlation_id: Uuid::now_v7(),
            start_event_id: Uuid::now_v7(),
            state: None,
        };

        let instance = row
            .try_into_instance::<TestSagaState, TestSagaStep>(
                SagaNameOwned::from(SagaName::new("test_saga")),
                CorrelationId::from(Uuid::now_v7()),
                Vec::new(),
            )
            .expect("row without state should deserialize");

        assert!(instance.state.is_none());
    }
}
