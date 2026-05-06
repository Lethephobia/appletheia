use super::OrganizationDescriptionChangeRejectionReason;

/// Describes the domain outcome of an organization operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationDescriptionChangeResult {
    Changed,
    Rejected {
        reason: OrganizationDescriptionChangeRejectionReason,
    },
}
