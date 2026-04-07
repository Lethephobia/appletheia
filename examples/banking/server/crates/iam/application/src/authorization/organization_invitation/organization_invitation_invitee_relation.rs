use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::OrganizationInvitation;

/// Allows the invited user to act on the invitation.
pub struct OrganizationInvitationInviteeRelation;

impl Relation for OrganizationInvitationInviteeRelation {
    const REF: RelationRef =
        RelationRef::new(OrganizationInvitation::TYPE, RelationName::new("invitee"));

    const EXPR: UsersetExpr = UsersetExpr::This;
}
