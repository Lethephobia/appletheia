use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::OrganizationFinanceManagerRelation;

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
        UsersetExpr::TupleToUserset {
            tupleset_relation: AccountOwnerRelation::REF,
            computed_userset: OrganizationFinanceManagerRelation::REF,
        },
    ]);
}
