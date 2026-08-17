use banking_ledger_domain::account::AccountOwner;
use banking_ledger_domain::currency::CurrencyId;

use crate::projection::MaterializedAccountStatus;

/// Search criteria for public account list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicAccountListCriteria {
    pub owner: Option<AccountOwner>,
    pub currency_id: Option<CurrencyId>,
    pub status_in: Option<Vec<MaterializedAccountStatus>>,
}

impl Default for PublicAccountListCriteria {
    fn default() -> Self {
        Self {
            owner: None,
            currency_id: None,
            status_in: Some(vec![MaterializedAccountStatus::Active]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_filters_to_active_accounts() {
        let criteria = PublicAccountListCriteria::default();

        assert_eq!(
            criteria.status_in,
            Some(vec![MaterializedAccountStatus::Active])
        );
    }
}
