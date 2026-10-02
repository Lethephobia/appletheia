use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::withdrawal::Withdrawal;

use super::WithdrawalAccountRelation;
use crate::authorization::AccountWithdrawalRequesterRelation;

pub struct WithdrawalNoteSetterRelation;

impl Relation for WithdrawalNoteSetterRelation {
    const REF: RelationRef = RelationRef::new(Withdrawal::TYPE, RelationName::new("note_setter"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: WithdrawalAccountRelation::REF.into(),
            computed_userset: AccountWithdrawalRequesterRelation::REF.into(),
        }
    }
}
