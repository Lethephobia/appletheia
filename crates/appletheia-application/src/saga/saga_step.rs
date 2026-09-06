use serde::{Serialize, de::DeserializeOwned};

/// Identifies one serializable logical step owned by a saga.
pub trait SagaStep: Copy + Eq + Send + Sync + Serialize + DeserializeOwned + 'static {}
