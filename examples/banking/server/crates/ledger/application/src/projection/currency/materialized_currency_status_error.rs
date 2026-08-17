use banking_ledger_domain::currency::CurrencyStatus;
use thiserror::Error;

/// Reports a domain status that cannot be materialized by a currency fragment.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum MaterializedCurrencyStatusError {
    #[error("currency status {0:?} cannot be materialized as a currency fragment status")]
    Unsupported(CurrencyStatus),
}
