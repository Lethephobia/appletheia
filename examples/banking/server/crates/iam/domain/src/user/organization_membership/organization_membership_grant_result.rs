use super::OrganizationMembershipGrantRejectionReason;

/// Describes the domain outcome of granting a user organization membership.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationMembershipGrantResult {
    Granted,
    Rejected {
        reason: OrganizationMembershipGrantRejectionReason,
    },
}
