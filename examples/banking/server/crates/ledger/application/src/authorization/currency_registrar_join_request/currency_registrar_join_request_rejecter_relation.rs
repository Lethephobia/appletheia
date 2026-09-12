use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{CurrencyRegistrarJoinRequest, CurrencyRegistrarJoinRequestRegistrarRelation};
use crate::CurrencyRegistrarMemberRelation;

/// Allows registrar administrators to reject join requests.
pub struct CurrencyRegistrarJoinRequestRejecterRelation;

impl Relation for CurrencyRegistrarJoinRequestRejecterRelation {
    const REF: RelationRef = RelationRef::new(
        CurrencyRegistrarJoinRequest::TYPE,
        RelationName::new("rejecter"),
    );

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: CurrencyRegistrarJoinRequestRegistrarRelation::REF.into(),
            computed_userset: CurrencyRegistrarMemberRelation::REF.into(),
        }
    }
}
