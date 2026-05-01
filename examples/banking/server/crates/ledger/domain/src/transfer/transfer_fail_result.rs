use super::TransferFailRejectionReason;

/// Describes the domain outcome of a transfer fail request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TransferFailResult {
    Failed,
    Rejected { reason: TransferFailRejectionReason },
}
