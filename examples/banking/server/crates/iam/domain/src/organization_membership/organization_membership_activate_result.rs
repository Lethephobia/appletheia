use super::OrganizationMembershipActivateRejectionReason;

/// Describes the domain outcome of an organization membership operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationMembershipActivateResult {
    Activated,
    Rejected {
        reason: OrganizationMembershipActivateRejectionReason,
    },
}
