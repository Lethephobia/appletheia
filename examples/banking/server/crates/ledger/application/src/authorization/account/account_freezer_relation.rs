use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::AccountStatusManagerRelation;

/// Allows status managers to freeze an account.
pub struct AccountFreezerRelation;

impl Relation for AccountFreezerRelation {
    const NAME: RelationName = RelationName::new("freezer");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(AccountStatusManagerRelation::NAME),
            },
        ])
    }
}
