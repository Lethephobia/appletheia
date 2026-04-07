use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Account, AccountOwnerRelation};

/// Allows owners to withdraw from an account.
pub struct AccountWithdrawerRelation;

impl Relation for AccountWithdrawerRelation {
    const REF: RelationRef = RelationRef::new(Account::TYPE, RelationName::new("withdrawer"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: AccountOwnerRelation::REF,
        },
    ]);
}
