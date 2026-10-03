use super::{ReadModelAttributeKey, ReadModelRelationshipKey};

/// Field allowlists selected after authorization; a filter does not grant access.
/// New fields remain hidden unless explicitly included in these lists.
pub trait ReadModelFieldFilter: Send + Sync {
    type AttributeKey: ReadModelAttributeKey;
    type RelationshipKey: ReadModelRelationshipKey;

    fn attribute_keys(&self) -> &[Self::AttributeKey];

    fn relationship_keys(&self) -> &[Self::RelationshipKey];
}
