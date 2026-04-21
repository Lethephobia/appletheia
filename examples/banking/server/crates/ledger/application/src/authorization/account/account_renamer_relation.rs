use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::OrganizationFinanceManagerRelation;

use super::{Account, AccountOwnerRelation};

/// Allows owners to rename an account.
pub struct AccountRenamerRelation;

impl Relation for AccountRenamerRelation {
    const REF: RelationRef = RelationRef::new(Account::TYPE, RelationName::new("renamer"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::ComputedUserset {
            relation: AccountOwnerRelation::REF,
        },
        UsersetExpr::TupleToUserset {
            tupleset_relation: AccountOwnerRelation::REF,
            computed_userset: OrganizationFinanceManagerRelation::REF,
        },
    ]);
}
