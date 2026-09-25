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

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: CurrencyRegistrarJoinRequestRegistrarRelation::REF.into(),
            computed_userset: CurrencyRegistrarMemberRelation::REF.into(),
        }
    }
}
