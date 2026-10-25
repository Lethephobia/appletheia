use super::ReadModelRef;
use serde::{Deserialize, Serialize};

/// Identifies a resource whose watchers need to re-run their queries.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadModelChange(ReadModelRef);

impl ReadModelChange {
    pub fn new(read_model_ref: ReadModelRef) -> Self {
        Self(read_model_ref)
    }

    pub fn read_model_ref(&self) -> &ReadModelRef {
        &self.0
    }
}

impl From<ReadModelRef> for ReadModelChange {
    fn from(value: ReadModelRef) -> Self {
        Self::new(value)
    }
}
