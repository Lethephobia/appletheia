use serde::Serialize;

use super::ReadModelResource;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ReadModelResourceDocumentData {
    Single(Option<ReadModelResource>),
    List(Vec<ReadModelResource>),
}
