use appletheia::application::authorization::{Relation, RelationName, UsersetExpr};

/// Allows the owning subject itself.
pub struct OrganizationOwnerRelation;

impl Relation for OrganizationOwnerRelation {
    const NAME: RelationName = RelationName::new("owner");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::This
    }
}
