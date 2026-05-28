use crate::currency::CurrencyId;

use super::{AccountName, AccountOwner};

/// Describes an account opening request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccountOpening {
    pub owner: AccountOwner,
    pub name: AccountName,
    pub currency_id: CurrencyId,
}

impl AccountOpening {
    pub(super) fn into_parts(self) -> (AccountOwner, AccountName, CurrencyId) {
        (self.owner, self.name, self.currency_id)
    }
}
