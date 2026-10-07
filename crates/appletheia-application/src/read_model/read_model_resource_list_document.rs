use serde::Serialize;

use super::ReadModelResource;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReadModelResourceListDocument {
    pub data: Vec<ReadModelResource>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub included: Vec<ReadModelResource>,
}
