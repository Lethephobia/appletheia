use appletheia::event_payload;

use crate::account::AccountId;
use crate::core::{CurrencyAmount, OnchainTransactionId};
use crate::token_binding::TokenBindingId;

use super::{
    DepositCompleteRejectionReason, DepositEventPayloadError, DepositFailRejectionReason,
    DepositFailureReason, DepositNote, DepositRequestRejectionReason,
    DepositSettlementVerifyRejectionReason,
};

/// Represents the domain events emitted by a `Deposit` aggregate.
#[event_payload(error = DepositEventPayloadError)]
pub enum DepositEventPayload {
    Requested {
        account_id: AccountId,
        token_binding_id: TokenBindingId,
        amount: CurrencyAmount,
        note: Option<DepositNote>,
    },
    RequestRejected {
        account_id: AccountId,
        token_binding_id: TokenBindingId,
        amount: CurrencyAmount,
        note: Option<DepositNote>,
        reason: DepositRequestRejectionReason,
    },
    SettlementVerified {
        account_id: AccountId,
        amount: CurrencyAmount,
        transaction_id: OnchainTransactionId,
    },
    SettlementVerifyRejected {
        transaction_id: OnchainTransactionId,
        reason: DepositSettlementVerifyRejectionReason,
    },
    Completed,
    CompleteRejected {
        reason: DepositCompleteRejectionReason,
    },
    Failed {
        reason: DepositFailureReason,
    },
    FailRejected {
        reason: DepositFailRejectionReason,
    },
}
