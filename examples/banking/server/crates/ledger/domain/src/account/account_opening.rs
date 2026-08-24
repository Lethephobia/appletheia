use crate::currency::CurrencyId;

use super::{AccountDescription, AccountName, AccountOwner};

/// Describes an account opening request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccountOpening {
    pub owner: AccountOwner,
    pub name: AccountName,
    pub description: Option<AccountDescription>,
    pub currency_id: CurrencyId,
}

impl AccountOpening {
    pub(super) fn into_parts(
        self,
    ) -> (
        AccountOwner,
        AccountName,
        Option<AccountDescription>,
        CurrencyId,
    ) {
        (self.owner, self.name, self.description, self.currency_id)
    }
}
