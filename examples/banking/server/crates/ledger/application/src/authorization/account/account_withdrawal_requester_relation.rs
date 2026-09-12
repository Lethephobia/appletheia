use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::OrganizationTreasurerRelation;

use super::{Account, AccountOwnerRelation};

/// Allows owners to request withdrawals from an account.
pub struct AccountWithdrawalRequesterRelation;

impl Relation for AccountWithdrawalRequesterRelation {
    const REF: RelationRef =
        RelationRef::new(Account::TYPE, RelationName::new("withdrawal_requester"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::ComputedUserset {
                relation: AccountOwnerRelation::REF.into(),
            },
            UsersetExpr::TupleToUserset {
                tupleset_relation: AccountOwnerRelation::REF.into(),
                computed_userset: OrganizationTreasurerRelation::REF.into(),
            },
        ])
    }
}
