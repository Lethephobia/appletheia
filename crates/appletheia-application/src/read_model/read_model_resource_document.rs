use serde::Serialize;

use super::{ReadModelResource, ReadModelResourceDocumentData};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReadModelResourceDocument {
    pub data: ReadModelResourceDocumentData,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub included: Vec<ReadModelResource>,
}
