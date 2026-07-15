use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::OrganizationTreasurerRelation;

use super::{Account, AccountOwnerRelation};

/// Allows owners to request withdrawals from an account.
pub struct AccountWithdrawalRequesterRelation;

impl Relation for AccountWithdrawalRequesterRelation {
    const REF: RelationRef =
        RelationRef::new(Account::TYPE, RelationName::new("withdrawal_requester"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::ComputedUserset {
            relation: AccountOwnerRelation::REF,
        },
        UsersetExpr::TupleToUserset {
            tupleset_relation: AccountOwnerRelation::REF,
            computed_userset: OrganizationTreasurerRelation::REF,
        },
    ]);
}
