use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::transfer::Transfer;

use super::TransferFromAccountRelation;
use crate::authorization::AccountTransactorRelation;

pub struct TransferFromAccountTransactorRelation;

impl Relation for TransferFromAccountTransactorRelation {
    const REF: RelationRef =
        RelationRef::new(Transfer::TYPE, RelationName::new("from_account_transactor"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: TransferFromAccountRelation::REF.into(),
            computed_userset: AccountTransactorRelation::REF.into(),
        }
    }
}
