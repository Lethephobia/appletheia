use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Account, AccountOwnerRelation};

/// Allows owners to rename an account.
pub struct AccountRenamerRelation;

impl Relation for AccountRenamerRelation {
    const REF: RelationRef = RelationRef::new(Account::TYPE, RelationName::new("renamer"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: AccountOwnerRelation::REF,
        },
    ]);
}
