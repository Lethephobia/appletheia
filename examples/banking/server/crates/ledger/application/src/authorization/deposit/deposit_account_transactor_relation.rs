use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::deposit::Deposit;

use super::DepositAccountRelation;
use crate::authorization::AccountTransactorRelation;

pub struct DepositAccountTransactorRelation;

impl Relation for DepositAccountTransactorRelation {
    const REF: RelationRef =
        RelationRef::new(Deposit::TYPE, RelationName::new("account_transactor"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: DepositAccountRelation::REF.into(),
            computed_userset: AccountTransactorRelation::REF.into(),
        }
    }
}
