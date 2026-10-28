use super::AccountOwnerRelation;
use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::organization::authorization::OrganizationOwnerRelation;
use banking_ledger_domain::account::Account;

pub struct AccountOwnershipManagerRelation;

impl Relation for AccountOwnershipManagerRelation {
    const REF: RelationRef =
        RelationRef::new(Account::TYPE, RelationName::new("ownership_manager"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::ComputedUserset {
                relation: AccountOwnerRelation::REF.into(),
            },
            UsersetExpr::TupleToUserset {
                tupleset_relation: AccountOwnerRelation::REF.into(),
                computed_userset: OrganizationOwnerRelation::REF.into(),
            },
        ])
    }
}
