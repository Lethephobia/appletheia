use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::AccountOwnerRelation;

/// Allows owners to withdraw from an account.
pub struct AccountWithdrawerRelation;

impl Relation for AccountWithdrawerRelation {
    const NAME: RelationName = RelationName::new("withdrawer");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(AccountOwnerRelation::NAME),
            },
        ])
    }
}
