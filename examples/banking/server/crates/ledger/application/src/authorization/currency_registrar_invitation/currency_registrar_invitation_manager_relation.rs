use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{CurrencyRegistrarInvitation, CurrencyRegistrarInvitationRegistrarRelation};
use crate::CurrencyRegistrarMemberRelation;

/// Allows registrar members to manage invitations.
pub struct CurrencyRegistrarInvitationManagerRelation;

impl Relation for CurrencyRegistrarInvitationManagerRelation {
    const REF: RelationRef = RelationRef::new(
        CurrencyRegistrarInvitation::TYPE,
        RelationName::new("manager"),
    );

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: CurrencyRegistrarInvitationRegistrarRelation::REF.into(),
            computed_userset: CurrencyRegistrarMemberRelation::REF.into(),
        }
    }
}
