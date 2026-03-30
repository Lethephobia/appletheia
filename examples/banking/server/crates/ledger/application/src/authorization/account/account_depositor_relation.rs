use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::AccountOwnerRelation;

/// Allows owners to deposit into an account.
pub struct AccountDepositorRelation;

impl Relation for AccountDepositorRelation {
    const NAME: RelationName = RelationName::new("depositor");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(AccountOwnerRelation::NAME),
            },
        ])
    }
}
