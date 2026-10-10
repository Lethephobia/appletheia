use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::withdrawal::Withdrawal;

use super::WithdrawalAccountRelation;
use crate::authorization::AccountOwnerRelation;

pub struct WithdrawalAccountOwnerRelation;

impl Relation for WithdrawalAccountOwnerRelation {
    const REF: RelationRef = RelationRef::new(Withdrawal::TYPE, RelationName::new("account_owner"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: WithdrawalAccountRelation::REF.into(),
            computed_userset: AccountOwnerRelation::REF.into(),
        }
    }
}
