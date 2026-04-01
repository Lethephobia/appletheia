use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::AccountOwnerRelation;

/// Allows owners to rename an account.
pub struct AccountRenamerRelation;

impl Relation for AccountRenamerRelation {
    const NAME: RelationName = RelationName::new("renamer");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(AccountOwnerRelation::NAME),
            },
        ])
    }
}
