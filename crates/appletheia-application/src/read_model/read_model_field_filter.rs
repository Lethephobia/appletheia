use super::ReadModel;

/// Field allowlists selected after authorization; a filter does not grant access.
/// New fields remain hidden unless explicitly included in these lists.
pub trait ReadModelFieldFilter<R>: Send + Sync
where
    R: ReadModel,
{
    fn attribute_keys(&self) -> &[R::AttributeKey];

    fn relationship_keys(&self) -> &[R::RelationshipKey];
}
