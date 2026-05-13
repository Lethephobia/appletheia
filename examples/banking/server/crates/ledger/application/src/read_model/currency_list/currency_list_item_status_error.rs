use banking_ledger_domain::currency::CurrencyStatus;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum CurrencyListItemStatusError {
    #[error("currency status {0:?} cannot be projected into a currency list status")]
    Unsupported(CurrencyStatus),
}
