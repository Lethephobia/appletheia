use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{CurrencyRegistrarJoinRequest, CurrencyRegistrarJoinRequestRequesterRelation};

/// Allows the requesting user to cancel their own join request.
pub struct CurrencyRegistrarJoinRequestCancelerRelation;

impl Relation for CurrencyRegistrarJoinRequestCancelerRelation {
    const REF: RelationRef = RelationRef::new(
        CurrencyRegistrarJoinRequest::TYPE,
        RelationName::new("canceler"),
    );

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::ComputedUserset {
            relation: CurrencyRegistrarJoinRequestRequesterRelation::REF.into(),
        }
    }
}
