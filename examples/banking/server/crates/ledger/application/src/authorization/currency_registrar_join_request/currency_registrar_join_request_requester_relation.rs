use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::CurrencyRegistrarJoinRequest;

/// Links a join request to the user who submitted membership.
pub struct CurrencyRegistrarJoinRequestRequesterRelation;

impl Relation for CurrencyRegistrarJoinRequestRequesterRelation {
    const REF: RelationRef = RelationRef::new(
        CurrencyRegistrarJoinRequest::TYPE,
        RelationName::new("requester"),
    );

    const EXPR: UsersetExpr = UsersetExpr::This;
}
