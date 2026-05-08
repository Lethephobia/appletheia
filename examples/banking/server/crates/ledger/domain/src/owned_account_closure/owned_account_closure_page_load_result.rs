use super::OwnedAccountClosurePageLoadRejectionReason;

/// Returned after loading an owned account closure page.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OwnedAccountClosurePageLoadResult {
    Loaded,
    Rejected {
        reason: OwnedAccountClosurePageLoadRejectionReason,
    },
}
