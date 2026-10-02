use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::deposit::Deposit;

use super::DepositAccountRelation;
use crate::authorization::AccountDepositRequesterRelation;

pub struct DepositNoteSetterRelation;

impl Relation for DepositNoteSetterRelation {
    const REF: RelationRef = RelationRef::new(Deposit::TYPE, RelationName::new("note_setter"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: DepositAccountRelation::REF.into(),
            computed_userset: AccountDepositRequesterRelation::REF.into(),
        }
    }
}
