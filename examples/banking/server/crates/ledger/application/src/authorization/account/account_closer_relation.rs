use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::AccountOwnerRelation;

/// Allows owners to close an account.
pub struct AccountCloserRelation;

impl Relation for AccountCloserRelation {
    const NAME: RelationName = RelationName::new("closer");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(AccountOwnerRelation::NAME),
            },
        ])
    }
}
