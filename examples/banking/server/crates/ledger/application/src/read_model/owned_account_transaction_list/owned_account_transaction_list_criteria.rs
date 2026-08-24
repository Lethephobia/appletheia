use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyCode;

use crate::projection::AccountTransactionStatus;

/// Search criteria for owned account transaction list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountTransactionListCriteria {
    pub account_id: Option<AccountId>,
    pub currency_code: Option<CurrencyCode>,
    pub status_in: Option<Vec<AccountTransactionStatus>>,
}
