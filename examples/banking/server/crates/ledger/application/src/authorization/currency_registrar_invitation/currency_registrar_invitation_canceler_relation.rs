use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{CurrencyRegistrarInvitation, CurrencyRegistrarInvitationRegistrarRelation};
use crate::CurrencyRegistrarMemberRelation;

/// Allows registrar inviters to cancel invitations.
pub struct CurrencyRegistrarInvitationCancelerRelation;

impl Relation for CurrencyRegistrarInvitationCancelerRelation {
    const REF: RelationRef = RelationRef::new(
        CurrencyRegistrarInvitation::TYPE,
        RelationName::new("canceler"),
    );

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: CurrencyRegistrarInvitationRegistrarRelation::REF.into(),
            computed_userset: CurrencyRegistrarMemberRelation::REF.into(),
        }
    }
}
