use super::OrganizationInvitationInviteeDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{OrganizationInvitation, User};

/// Allows the invited user to act on the invitation.
pub struct OrganizationInvitationInviteeRelation;

impl Relation for OrganizationInvitationInviteeRelation {
    const REF: RelationRef =
        RelationRef::new(OrganizationInvitation::TYPE, RelationName::new("invitee"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<
            OrganizationInvitation,
            _,
            OrganizationInvitationInviteeDerivationHandlerError,
        >(|aggregate| {
            let mut entries = RelationshipEntries::new();
            entries.insert::<OrganizationInvitation, User>(
                aggregate.aggregate_id(),
                *aggregate.invitee_id()?,
            );
            Ok(entries)
        })
    }
}
