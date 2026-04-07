use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Account, AccountOwnerRelation};

/// Allows owners to deposit into an account.
pub struct AccountDepositorRelation;

impl Relation for AccountDepositorRelation {
    const REF: RelationRef = RelationRef::new(Account::TYPE, RelationName::new("depositor"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: AccountOwnerRelation::REF,
        },
    ]);
}
