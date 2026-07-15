use banking_ledger_domain::account::{AccountId, AccountOwner};
use banking_ledger_domain::currency::CurrencyId;

use super::PublicAccountListItemStatus;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicAccountListAccountUpsert {
    pub id: AccountId,
    pub owner: AccountOwner,
    pub currency_id: CurrencyId,
    pub status: PublicAccountListItemStatus,
}
