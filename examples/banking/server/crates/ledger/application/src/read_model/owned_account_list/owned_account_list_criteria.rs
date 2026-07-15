use banking_ledger_domain::currency::CurrencyId;

use super::OwnedAccountListItemStatus;

/// Search criteria for owned account list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountListCriteria {
    pub currency_id: Option<CurrencyId>,
    pub status: Option<OwnedAccountListItemStatus>,
}
