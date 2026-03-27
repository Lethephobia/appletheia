use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::UserStatusManagerRelation;

/// Allows status managers to remove a user.
pub struct UserRemoverRelation;

impl Relation for UserRemoverRelation {
    const NAME: RelationName = RelationName::new("remover");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(UserStatusManagerRelation::NAME),
            },
        ])
    }
}
