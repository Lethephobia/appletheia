use crate::request_context::CorrelationId;

use super::SagaInstanceId;
use super::SagaName;
use super::SagaState;

#[derive(Clone, Debug, PartialEq)]
pub struct SagaInstance<N: SagaName, S: SagaState> {
    pub saga_instance_id: SagaInstanceId,
    pub saga_name: N,
    pub correlation_id: CorrelationId,
    pub status: SagaStatus<S>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SagaStatus<S: SagaState> {
    InProgress { state: Option<S> },
    Succeeded { state: S },
    Failed { state: S, error: serde_json::Value },
}

impl<N: SagaName, S: SagaState> SagaInstance<N, S> {
    pub fn new(saga_name: N, correlation_id: CorrelationId) -> Self {
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
