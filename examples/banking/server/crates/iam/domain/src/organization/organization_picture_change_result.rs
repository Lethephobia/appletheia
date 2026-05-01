use super::OrganizationPictureChangeRejectionReason;

/// Describes the domain outcome of an organization operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationPictureChangeResult {
    Changed,
    Rejected {
        reason: OrganizationPictureChangeRejectionReason,
    },
}
