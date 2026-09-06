use appletheia::event_payload;

use crate::account::AccountId;
use crate::core::{CurrencyAmount, OnchainTransactionId, TokenOwnerAddress};
use crate::token_binding::TokenBindingId;

use super::{WithdrawalEventPayloadError, WithdrawalFailureReason, WithdrawalNote};

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
    SettlementExecuted {
        transaction_id: OnchainTransactionId,
    },
    Completed,
    Failed {
        reason: WithdrawalFailureReason,
    },
}
