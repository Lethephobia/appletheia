use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::deposit::Deposit;

use super::DepositAccountRelation;
use crate::authorization::AccountOwnerRelation;

pub struct DepositAccountOwnerRelation;

impl Relation for DepositAccountOwnerRelation {
    const REF: RelationRef = RelationRef::new(Deposit::TYPE, RelationName::new("account_owner"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: DepositAccountRelation::REF.into(),
            computed_userset: AccountOwnerRelation::REF.into(),
        }
    }
}
