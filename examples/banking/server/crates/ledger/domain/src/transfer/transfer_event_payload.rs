use appletheia::event_payload;

use crate::account::AccountId;
use crate::core::CurrencyAmount;

use super::{TransferEventPayloadError, TransferFailureReason, TransferNote};

/// Represents the domain events emitted by a `Transfer` aggregate.
#[event_payload(error = TransferEventPayloadError)]
pub enum TransferEventPayload {
    Requested {
        from_account_id: AccountId,
        to_account_id: AccountId,
        amount: CurrencyAmount,
        note: Option<TransferNote>,
    },
    Completed,
    Failed {
        reason: TransferFailureReason,
    },
}
