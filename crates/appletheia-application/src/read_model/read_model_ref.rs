use serde::Serialize;

use super::{ReadModel, ReadModelId, ReadModelType};

/// Identifies the same resource regardless of its selected fields.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct ReadModelRef {
    #[serde(rename = "type")]
    pub read_model_type: ReadModelType,
    pub id: ReadModelId,
}

impl ReadModelRef {
    pub fn new<R>(id: R::Id) -> Self
    where
        R: ReadModel,
    {
        Self {
            read_model_type: R::TYPE,
            id: ReadModelId::new(id.to_string()),
        }
    }
}
