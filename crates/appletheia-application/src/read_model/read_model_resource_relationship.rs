use serde::Serialize;

use super::ReadModelRelationshipData;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReadModelResourceRelationship {
    pub data: ReadModelRelationshipData,
}
