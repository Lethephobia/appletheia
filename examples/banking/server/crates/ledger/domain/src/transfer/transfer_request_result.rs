use super::TransferRequestRejectionReason;

/// Describes the domain outcome of a transfer request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TransferRequestResult {
    Requested,
    Rejected {
        reason: TransferRequestRejectionReason,
    },
}
