use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{CurrencyRegistrarJoinRequest, CurrencyRegistrarJoinRequestRegistrarRelation};
use crate::CurrencyRegistrarMemberRelation;

/// Allows registrar members to manage join requests.
pub struct CurrencyRegistrarJoinRequestManagerRelation;

impl Relation for CurrencyRegistrarJoinRequestManagerRelation {
    const REF: RelationRef = RelationRef::new(
        CurrencyRegistrarJoinRequest::TYPE,
        RelationName::new("manager"),
    );

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: CurrencyRegistrarJoinRequestRegistrarRelation::REF.into(),
            computed_userset: CurrencyRegistrarMemberRelation::REF.into(),
        }
    }
}
