use banking_ledger_domain::core::CurrencyCode;

use crate::projection::MaterializedAccountStatus;

/// Search criteria for owned account list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountListCriteria {
    pub currency_code: Option<CurrencyCode>,
    pub status_in: Option<Vec<MaterializedAccountStatus>>,
}
