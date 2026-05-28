use crate::{OrganizationId, OrganizationMembershipRoles, UserId};

use super::{OrganizationInvitationExpiresAt, OrganizationInvitationIssuer};

/// Describes an organization invitation issuance request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrganizationInvitationIssuance {
    pub organization_id: OrganizationId,
    pub invitee_id: UserId,
    pub roles: OrganizationMembershipRoles,
    pub issuer: OrganizationInvitationIssuer,
    pub expires_at: OrganizationInvitationExpiresAt,
}

impl OrganizationInvitationIssuance {
    /// Returns the expiration timestamp.
    pub fn expires_at(&self) -> &OrganizationInvitationExpiresAt {
        &self.expires_at
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        OrganizationId,
        UserId,
        OrganizationMembershipRoles,
        OrganizationInvitationIssuer,
        OrganizationInvitationExpiresAt,
    ) {
        (
            self.organization_id,
            self.invitee_id,
            self.roles,
            self.issuer,
            self.expires_at,
        )
    }
}
