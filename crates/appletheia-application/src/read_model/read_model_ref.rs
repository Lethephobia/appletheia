use serde::{Deserialize, Serialize};

use super::{ReadModel, ReadModelId, ReadModelTypeOwned};

/// Identifies the same resource regardless of its selected fields.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ReadModelRef {
    #[serde(rename = "type")]
    pub read_model_type: ReadModelTypeOwned,
    pub id: ReadModelId,
}

impl ReadModelRef {
    pub fn new<R>(id: R::Id) -> Self
    where
        R: ReadModel,
    {
        Self {
            read_model_type: R::TYPE.into(),
            id: ReadModelId::new(id.to_string()),
        }
    }
}
