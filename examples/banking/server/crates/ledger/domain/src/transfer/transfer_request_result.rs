use super::{TransferId, TransferRequestRejectionReason};

/// Describes the domain outcome of a transfer request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TransferRequestResult {
    Requested {
        transfer_id: TransferId,
    },
    Rejected {
        transfer_id: TransferId,
        reason: TransferRequestRejectionReason,
    },
}
