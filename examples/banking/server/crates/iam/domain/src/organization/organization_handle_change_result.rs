use super::OrganizationHandleChangeRejectionReason;

/// Describes the domain outcome of an organization operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationHandleChangeResult {
    Changed,
    Rejected {
        reason: OrganizationHandleChangeRejectionReason,
    },
}
