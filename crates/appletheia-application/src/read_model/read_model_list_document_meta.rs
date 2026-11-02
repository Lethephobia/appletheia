use serde::Serialize;

use super::ReadModelPagination;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReadModelListDocumentMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<ReadModelPagination>,
}
