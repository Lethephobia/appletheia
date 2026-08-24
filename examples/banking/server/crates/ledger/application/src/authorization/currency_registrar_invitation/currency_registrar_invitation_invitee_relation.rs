use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::CurrencyRegistrarInvitation;

/// Allows the invited user to act on the invitation.
pub struct CurrencyRegistrarInvitationInviteeRelation;

impl Relation for CurrencyRegistrarInvitationInviteeRelation {
    const REF: RelationRef = RelationRef::new(
        CurrencyRegistrarInvitation::TYPE,
        RelationName::new("invitee"),
    );

    const EXPR: UsersetExpr = UsersetExpr::This;
}
