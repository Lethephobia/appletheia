use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Account, AccountOwnerRelation};

/// Allows owners to request transfers from an account.
pub struct AccountTransferRequesterRelation;

impl Relation for AccountTransferRequesterRelation {
    const REF: RelationRef =
        RelationRef::new(Account::TYPE, RelationName::new("transfer_requester"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: AccountOwnerRelation::REF,
        },
    ]);
}
