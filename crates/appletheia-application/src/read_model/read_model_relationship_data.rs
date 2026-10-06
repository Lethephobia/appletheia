use serde::Serialize;

use super::ReadModelRef;

/// Relationship linkage. Serializes directly as an identifier, null, or an array.
/// This represents the `data` member, not the surrounding relationship object.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ReadModelRelationshipData {
    ToOne(Option<ReadModelRef>),
    ToMany(Vec<ReadModelRef>),
}
