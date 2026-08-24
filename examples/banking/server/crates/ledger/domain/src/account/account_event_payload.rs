use appletheia::event_payload;

use crate::core::CurrencyAmount;
use crate::currency::CurrencyId;

use super::{
    AccountCloseRejectionReason, AccountDepositRejectionReason, AccountDescription,
    AccountDescriptionChangeRejectionReason, AccountEventPayloadError,
    AccountFreezeRejectionReason, AccountFundsReserveRejectionReason, AccountName,
    AccountNameChangeRejectionReason, AccountOwner, AccountOwnershipTransferRejectionReason,
    AccountReservedFundsCommitRejectionReason, AccountReservedFundsReleaseRejectionReason,
    AccountThawRejectionReason, AccountWithdrawRejectionReason,
};

/// Represents the domain events emitted by an `Account` aggregate.
#[event_payload(error = AccountEventPayloadError)]
pub enum AccountEventPayload {
    Opened {
        owner: AccountOwner,
        name: AccountName,
        description: Option<AccountDescription>,
        currency_id: CurrencyId,
    },
    OwnershipTransferred {
        owner: AccountOwner,
    },
    OwnershipTransferRejected {
        owner: AccountOwner,
        reason: AccountOwnershipTransferRejectionReason,
    },
    NameChanged {
        name: AccountName,
    },
    NameChangeRejected {
        name: AccountName,
        reason: AccountNameChangeRejectionReason,
    },
    DescriptionChanged {
        description: Option<AccountDescription>,
    },
    DescriptionChangeRejected {
        description: Option<AccountDescription>,
        reason: AccountDescriptionChangeRejectionReason,
    },
    Deposited {
        amount: CurrencyAmount,
    },
    DepositRejected {
        amount: CurrencyAmount,
        reason: AccountDepositRejectionReason,
    },
    Withdrawn {
        amount: CurrencyAmount,
    },
    WithdrawRejected {
        amount: CurrencyAmount,
        reason: AccountWithdrawRejectionReason,
    },
    FundsReserved {
        amount: CurrencyAmount,
    },
    FundsReserveRejected {
        amount: CurrencyAmount,
        reason: AccountFundsReserveRejectionReason,
    },
    ReservedFundsReleased {
        amount: CurrencyAmount,
    },
    ReservedFundsReleaseRejected {
        amount: CurrencyAmount,
        reason: AccountReservedFundsReleaseRejectionReason,
    },
    ReservedFundsCommitted {
        amount: CurrencyAmount,
    },
    ReservedFundsCommitRejected {
        amount: CurrencyAmount,
        reason: AccountReservedFundsCommitRejectionReason,
    },
    Frozen,
    FreezeRejected {
        reason: AccountFreezeRejectionReason,
    },
    Thawed,
    ThawRejected {
        reason: AccountThawRejectionReason,
    },
    Closed,
    CloseRejected {
        reason: AccountCloseRejectionReason,
    },
}
