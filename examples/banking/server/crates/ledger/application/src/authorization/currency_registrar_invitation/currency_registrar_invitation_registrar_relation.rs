use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::CurrencyRegistrarInvitation;

/// Links an invitation to its registrar.
pub struct CurrencyRegistrarInvitationRegistrarRelation;

impl Relation for CurrencyRegistrarInvitationRegistrarRelation {
    const REF: RelationRef = RelationRef::new(
        CurrencyRegistrarInvitation::TYPE,
        RelationName::new("registrar"),
    );

    const EXPR: UsersetExpr = UsersetExpr::This;
}
