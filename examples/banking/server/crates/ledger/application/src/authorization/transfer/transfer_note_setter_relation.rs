use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::transfer::Transfer;

use super::TransferFromAccountRelation;
use crate::authorization::AccountTransferRequesterRelation;

pub struct TransferNoteSetterRelation;

impl Relation for TransferNoteSetterRelation {
    const REF: RelationRef = RelationRef::new(Transfer::TYPE, RelationName::new("note_setter"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: TransferFromAccountRelation::REF.into(),
            computed_userset: AccountTransferRequesterRelation::REF.into(),
        }
    }
}
