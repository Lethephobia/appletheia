use serde::Serialize;

use super::{ReadModelResource, ReadModelResourceListDocumentMeta};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ReadModelResourceDocument {
    Single {
        data: Option<ReadModelResource>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        included: Vec<ReadModelResource>,
    },
    List {
        data: Vec<ReadModelResource>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        included: Vec<ReadModelResource>,
        #[serde(skip_serializing_if = "Option::is_none")]
        meta: Option<ReadModelResourceListDocumentMeta>,
    },
}
