use appletheia::application::authorization::{Relation, RelationName, UsersetExpr};

/// Allows the owning user itself.
pub struct AccountOwnerRelation;

impl Relation for AccountOwnerRelation {
    const NAME: RelationName = RelationName::new("owner");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::This
    }
}
