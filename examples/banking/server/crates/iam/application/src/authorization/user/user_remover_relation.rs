use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::{UserOwnerRelation, UserStatusManagerRelation};

/// Allows owners to remove a user.
pub struct UserRemoverRelation;

impl Relation for UserRemoverRelation {
    const NAME: RelationName = RelationName::new("remover");

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
