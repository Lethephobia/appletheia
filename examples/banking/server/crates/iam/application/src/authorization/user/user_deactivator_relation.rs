use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::{UserOwnerRelation, UserStatusManagerRelation};

/// Allows owners to deactivate a user.
pub struct UserDeactivatorRelation;

impl Relation for UserDeactivatorRelation {
    const NAME: RelationName = RelationName::new("deactivator");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(UserOwnerRelation::NAME),
            },
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(UserStatusManagerRelation::NAME),
            },
        ])
    }
}
