use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::CurrencyRegistrarJoinRequest;

/// Links a join request to its registrar.
pub struct CurrencyRegistrarJoinRequestRegistrarRelation;

impl Relation for CurrencyRegistrarJoinRequestRegistrarRelation {
    const REF: RelationRef = RelationRef::new(
        CurrencyRegistrarJoinRequest::TYPE,
        RelationName::new("registrar"),
    );

    const EXPR: UsersetExpr = UsersetExpr::This;
}
