use appletheia::application::authorization::{Relation, RelationName, UsersetExpr};

/// Allows the invited user to act on the invitation.
pub struct OrganizationInvitationInviteeRelation;

impl Relation for OrganizationInvitationInviteeRelation {
    const NAME: RelationName = RelationName::new("invitee");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::This
    }
}
