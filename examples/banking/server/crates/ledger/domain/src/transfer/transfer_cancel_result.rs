use super::TransferCancelRejectionReason;

/// Describes the domain outcome of a transfer cancel request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TransferCancelResult {
    Cancelled,
    Rejected {
        reason: TransferCancelRejectionReason,
    },
}
