use serde::Serialize;

use super::ReadModelResourceWindow;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReadModelResourceListDocumentMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<ReadModelResourceWindow>,
}
