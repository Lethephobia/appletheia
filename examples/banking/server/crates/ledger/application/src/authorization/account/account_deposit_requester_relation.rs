use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::OrganizationTreasurerRelation;

use super::{Account, AccountOwnerRelation};

/// Allows owners to request deposits into an account.
pub struct AccountDepositRequesterRelation;

impl Relation for AccountDepositRequesterRelation {
    const REF: RelationRef =
        RelationRef::new(Account::TYPE, RelationName::new("deposit_requester"));

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
