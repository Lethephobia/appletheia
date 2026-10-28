use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::withdrawal::Withdrawal;

use super::WithdrawalAccountRelation;
use crate::account::authorization::AccountTransactorRelation;

pub struct WithdrawalAccountTransactorRelation;

impl Relation for WithdrawalAccountTransactorRelation {
    const REF: RelationRef =
        RelationRef::new(Withdrawal::TYPE, RelationName::new("account_transactor"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: WithdrawalAccountRelation::REF.into(),
            computed_userset: AccountTransactorRelation::REF.into(),
        }
    }
}
