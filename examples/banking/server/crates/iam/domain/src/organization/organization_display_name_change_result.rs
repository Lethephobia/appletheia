use super::OrganizationDisplayNameChangeRejectionReason;

/// Describes the domain outcome of an organization operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationDisplayNameChangeResult {
    Changed,
    Rejected {
        reason: OrganizationDisplayNameChangeRejectionReason,
    },
}
