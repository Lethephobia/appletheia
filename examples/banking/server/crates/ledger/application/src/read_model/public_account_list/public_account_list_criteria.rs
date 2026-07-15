use banking_ledger_domain::account::AccountOwner;
use banking_ledger_domain::currency::CurrencyId;

/// Search criteria for public account list reads.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PublicAccountListCriteria {
    pub owner: Option<AccountOwner>,
    pub currency_id: Option<CurrencyId>,
}
