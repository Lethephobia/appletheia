use serde::{Deserialize, Serialize};

use super::{SagaInstanceId, SagaNameOwned, SerializedSagaStep};

/// Identifies the saga instance that dispatched an asynchronous command.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SagaCommandOrigin {
    pub saga_name: SagaNameOwned,
    pub saga_instance_id: SagaInstanceId,
    pub step: SerializedSagaStep,
}
