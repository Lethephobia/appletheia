use super::{OrganizationMembershipCreateRejectionReason, OrganizationMembershipId};

/// Describes the domain outcome of an organization membership create request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationMembershipCreateResult {
    Created {
        organization_membership_id: OrganizationMembershipId,
    },
    Rejected {
        reason: OrganizationMembershipCreateRejectionReason,
    },
}
