use super::OrganizationWebsiteUrlChangeRejectionReason;

/// Describes the domain outcome of an organization operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationWebsiteUrlChangeResult {
    Changed,
    Rejected {
        reason: OrganizationWebsiteUrlChangeRejectionReason,
    },
}
