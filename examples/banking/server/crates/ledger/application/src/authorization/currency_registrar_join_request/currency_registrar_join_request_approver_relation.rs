use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{CurrencyRegistrarJoinRequest, CurrencyRegistrarJoinRequestRegistrarRelation};
use crate::CurrencyRegistrarMemberRelation;

/// Allows registrar administrators to approve join requests.
pub struct CurrencyRegistrarJoinRequestApproverRelation;

impl Relation for CurrencyRegistrarJoinRequestApproverRelation {
    const REF: RelationRef = RelationRef::new(
        CurrencyRegistrarJoinRequest::TYPE,
        RelationName::new("approver"),
    );

    const EXPR: UsersetExpr = UsersetExpr::TupleToUserset {
        tupleset_relation: CurrencyRegistrarJoinRequestRegistrarRelation::REF,
        computed_userset: CurrencyRegistrarMemberRelation::REF,
    };
}
