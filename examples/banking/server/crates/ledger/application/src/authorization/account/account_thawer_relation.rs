use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::AccountStatusManagerRelation;

/// Allows status managers to thaw an account.
pub struct AccountThawerRelation;

impl Relation for AccountThawerRelation {
    const NAME: RelationName = RelationName::new("thawer");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(AccountStatusManagerRelation::NAME),
            },
        ])
    }
}
