use appletheia::event_payload;

use crate::account::AccountId;
use crate::core::{CurrencyAmount, OnchainTransactionId, TokenOwnerAddress};
use crate::token_binding::TokenBindingId;

use super::{
    WithdrawalCompleteRejectionReason, WithdrawalEventPayloadError, WithdrawalFailRejectionReason,
    WithdrawalFailureReason, WithdrawalNote, WithdrawalRequestRejectionReason,
    WithdrawalSettlementExecuteRejectionReason,
};

/// Represents the domain events emitted by a `Withdrawal` aggregate.
#[event_payload(error = WithdrawalEventPayloadError)]
pub enum WithdrawalEventPayload {
    Requested {
        account_id: AccountId,
        token_binding_id: TokenBindingId,
        token_owner_address: TokenOwnerAddress,
        amount: CurrencyAmount,
        note: Option<WithdrawalNote>,
    },
    RequestRejected {
        account_id: AccountId,
        token_binding_id: TokenBindingId,
        token_owner_address: TokenOwnerAddress,
        amount: CurrencyAmount,
        note: Option<WithdrawalNote>,
        reason: WithdrawalRequestRejectionReason,
    },
    SettlementExecuted {
        transaction_id: OnchainTransactionId,
    },
    SettlementExecuteRejected {
        transaction_id: Option<OnchainTransactionId>,
        reason: WithdrawalSettlementExecuteRejectionReason,
    },
    Completed,
    CompleteRejected {
        reason: WithdrawalCompleteRejectionReason,
    },
    Failed {
        reason: WithdrawalFailureReason,
    },
    FailRejected {
        reason: WithdrawalFailRejectionReason,
    },
}
