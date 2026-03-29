use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::UserStatusManagerRelation;

/// Allows status managers to activate a user.
pub struct UserActivatorRelation;

impl Relation for UserActivatorRelation {
    const NAME: RelationName = RelationName::new("activator");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(UserStatusManagerRelation::NAME),
            },
        ])
    }
}
