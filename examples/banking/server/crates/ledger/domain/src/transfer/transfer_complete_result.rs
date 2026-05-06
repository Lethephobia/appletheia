use super::TransferCompleteRejectionReason;

/// Describes the domain outcome of a transfer complete request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TransferCompleteResult {
    Completed,
    Rejected {
        reason: TransferCompleteRejectionReason,
    },
}
