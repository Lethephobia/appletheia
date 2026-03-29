use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::UserOwnerRelation;

/// Allows subjects with a direct tuple to manage user status transitions.
pub struct UserStatusManagerRelation;

impl Relation for UserStatusManagerRelation {
    const NAME: RelationName = RelationName::new("status_manager");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Difference {
            base: Box::new(UsersetExpr::This),
            subtract: Box::new(UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(UserOwnerRelation::NAME),
            }),
        }
    }
}
