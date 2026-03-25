use appletheia::application::authorization::{Relation, RelationName, UsersetExpr};

/// Defines the `assignee` relation for `Role`.
pub struct RoleAssigneeRelation;

impl Relation for RoleAssigneeRelation {
    const NAME: RelationName = RelationName::new("assignee");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::This
    }
}
