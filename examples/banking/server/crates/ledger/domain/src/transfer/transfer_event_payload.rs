use appletheia::event_payload;

use crate::account::AccountId;
use crate::core::CurrencyAmount;

use super::{
    TransferCompleteRejectionReason, TransferEventPayloadError, TransferFailRejectionReason,
    TransferFailureReason, TransferNote, TransferRequestRejectionReason,
};

/// Represents the domain events emitted by a `Transfer` aggregate.
#[event_payload(error = TransferEventPayloadError)]
pub enum TransferEventPayload {
    Requested {
        from_account_id: AccountId,
        to_account_id: AccountId,
        amount: CurrencyAmount,
        note: Option<TransferNote>,
    },
    RequestRejected {
        from_account_id: AccountId,
        to_account_id: AccountId,
        amount: CurrencyAmount,
        note: Option<TransferNote>,
        reason: TransferRequestRejectionReason,
    },
    Completed,
    CompleteRejected {
        reason: TransferCompleteRejectionReason,
    },
    Failed {
        reason: TransferFailureReason,
    },
    FailRejected {
        reason: TransferFailRejectionReason,
    },
}
