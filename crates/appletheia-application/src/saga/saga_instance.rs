use crate::request_context::CorrelationId;

use super::SagaInstanceId;
use super::SagaNameOwned;
use super::SagaState;

#[derive(Clone, Debug, PartialEq)]
pub struct SagaInstance<S: SagaState> {
    pub saga_instance_id: SagaInstanceId,
    pub saga_name: SagaNameOwned,
    pub correlation_id: CorrelationId,
    pub status: SagaStatus<S>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SagaStatus<S: SagaState> {
    InProgress { state: Option<S> },
    Succeeded { state: S },
    Failed { state: S, error: serde_json::Value },
}

impl<S: SagaState> SagaInstance<S> {
    pub fn new(saga_name: SagaNameOwned, correlation_id: CorrelationId) -> Self {
        Self {
            saga_instance_id: SagaInstanceId::new(),
            saga_name,
            correlation_id,
            status: SagaStatus::InProgress { state: None },
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self.status,
            SagaStatus::Succeeded { .. } | SagaStatus::Failed { .. }
        )
    }

    pub fn is_succeeded(&self) -> bool {
        matches!(self.status, SagaStatus::Succeeded { .. })
    }

    pub fn is_failed(&self) -> bool {
        matches!(self.status, SagaStatus::Failed { .. })
    }
}
