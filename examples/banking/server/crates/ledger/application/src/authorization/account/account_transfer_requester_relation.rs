use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::AccountOwnerRelation;

/// Allows owners to request transfers from an account.
pub struct AccountTransferRequesterRelation;

impl Relation for AccountTransferRequesterRelation {
    const NAME: RelationName = RelationName::new("transfer_requester");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(AccountOwnerRelation::NAME),
            },
        ])
    }
}
