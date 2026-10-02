use appletheia::event_payload;

use crate::core::CurrencyAmount;
use crate::currency::CurrencyId;

use super::{AccountDescription, AccountEventPayloadError, AccountName, AccountOwner};

/// Represents the domain events emitted by an `Account` aggregate.
#[event_payload(error = AccountEventPayloadError)]
pub enum AccountEventPayload {
    Opened {
        owner: AccountOwner,
        name: AccountName,
        currency_id: CurrencyId,
    },
    OwnershipTransferred {
        owner: AccountOwner,
    },
    NameChanged {
        name: AccountName,
    },
    DescriptionSet {
        description: Option<AccountDescription>,
    },
    Deposited {
        amount: CurrencyAmount,
    },
    Withdrawn {
        amount: CurrencyAmount,
    },
    FundsReserved {
        amount: CurrencyAmount,
    },
    ReservedFundsReleased {
        amount: CurrencyAmount,
    },
    ReservedFundsCommitted {
        amount: CurrencyAmount,
    },
    Frozen,
    Thawed,
    Closed,
}
