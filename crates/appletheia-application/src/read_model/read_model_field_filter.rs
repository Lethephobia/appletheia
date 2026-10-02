use super::{ReadModelAttributeKey, ReadModelRelationshipKey};

/// Field allowlists selected after authorization; a filter does not grant access.
/// New fields remain hidden unless explicitly included in these lists.
pub trait ReadModelFieldFilter: Send + Sync {
    fn attribute_keys(&self) -> &[ReadModelAttributeKey];

    fn relationship_keys(&self) -> &[ReadModelRelationshipKey];
}
