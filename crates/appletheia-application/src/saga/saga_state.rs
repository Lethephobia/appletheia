use serde::{Serialize, de::DeserializeOwned};

pub trait SagaState: Serialize + DeserializeOwned + Send + Sync + 'static {}
