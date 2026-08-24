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

    const EXPR: UsersetExpr = UsersetExpr::TupleToUserset {
        tupleset_relation: CurrencyRegistrarJoinRequestRegistrarRelation::REF,
        computed_userset: CurrencyRegistrarMemberRelation::REF,
    };
}
