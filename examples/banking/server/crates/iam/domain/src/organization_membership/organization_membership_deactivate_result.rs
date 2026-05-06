use super::OrganizationMembershipDeactivateRejectionReason;

/// Describes the domain outcome of an organization membership operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationMembershipDeactivateResult {
    Deactivated,
    Rejected {
        reason: OrganizationMembershipDeactivateRejectionReason,
    },
}
