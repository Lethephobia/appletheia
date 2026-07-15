use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::currency::CurrencyId;

use super::OwnedAccountTransactionListItemStatus;

/// Search criteria for owned account transaction list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountTransactionListCriteria {
    pub account_id: Option<AccountId>,
    pub currency_id: Option<CurrencyId>,
    pub status: Option<OwnedAccountTransactionListItemStatus>,
}
