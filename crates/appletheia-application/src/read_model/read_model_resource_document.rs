use serde::Serialize;

use super::ReadModelResource;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReadModelResourceDocument {
    pub data: Option<ReadModelResource>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub included: Vec<ReadModelResource>,
}
