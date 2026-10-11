use super::AccountOwnerRelation;
use crate::authorization::OrganizationTreasurerRelation;
use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::Account;

pub struct AccountTransactorRelation;

impl Relation for AccountTransactorRelation {
    const REF: RelationRef = RelationRef::new(Account::TYPE, RelationName::new("transactor"));

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
