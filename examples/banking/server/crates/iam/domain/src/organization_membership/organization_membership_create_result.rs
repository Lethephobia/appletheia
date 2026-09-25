use super::OrganizationMembershipCreateRejectionReason;

/// Describes the domain outcome of creating an organization membership.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationMembershipCreateResult {
    Created,
    Rejected {
        reason: OrganizationMembershipCreateRejectionReason,
    },
}
