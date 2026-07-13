use appletheia::event_payload;

use crate::account::AccountId;
use crate::core::{CurrencyAmount, TokenAccountOwnerAddress};
use crate::currency::CurrencyId;

use super::{
    DepositCompleteRejectionReason, DepositEventPayloadError, DepositFailRejectionReason,
    DepositFailureReason, DepositId, DepositRequestRejectionReason,
    DepositTokenTransferRecordRejectionReason,
};

/// Represents the domain events emitted by a `Deposit` aggregate.
#[event_payload(error = DepositEventPayloadError)]
pub enum DepositEventPayload {
    Requested {
        id: DepositId,
        account_id: AccountId,
        currency_id: CurrencyId,
        token_account_owner_address: TokenAccountOwnerAddress,
        amount: CurrencyAmount,
    },
    RequestRejected {
        id: DepositId,
        account_id: AccountId,
        currency_id: CurrencyId,
        token_account_owner_address: TokenAccountOwnerAddress,
        amount: CurrencyAmount,
        reason: DepositRequestRejectionReason,
    },
    TokenTransferred {
        id: DepositId,
        account_id: AccountId,
        amount: CurrencyAmount,
    },
    TokenTransferRecordRejected {
        reason: DepositTokenTransferRecordRejectionReason,
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
