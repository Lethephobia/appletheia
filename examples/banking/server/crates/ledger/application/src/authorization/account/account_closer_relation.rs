use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Account, AccountOwnerRelation};

/// Allows owners to close an account.
pub struct AccountCloserRelation;

impl Relation for AccountCloserRelation {
    const REF: RelationRef = RelationRef::new(Account::TYPE, RelationName::new("closer"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: AccountOwnerRelation::REF,
        },
    ]);
}
