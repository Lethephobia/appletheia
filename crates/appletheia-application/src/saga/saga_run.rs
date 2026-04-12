use serde::{Serialize, de::DeserializeOwned};

use crate::request_context::CorrelationId;

use super::{SagaNameOwned, SagaRunId};

#[derive(Clone, Debug, PartialEq)]
pub struct SagaRun<C>
where
    C: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    pub saga_run_id: SagaRunId,
    pub saga_name: SagaNameOwned,
    pub correlation_id: CorrelationId,
    pub context: C,
}

impl<C> SagaRun<C>
where
    C: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    pub fn new(saga_name: SagaNameOwned, correlation_id: CorrelationId, context: C) -> Self {
        Self {
            saga_run_id: SagaRunId::new(),
            saga_name,
            correlation_id,
            context,
        }
    }
}
