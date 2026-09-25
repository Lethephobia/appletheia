use banking_ledger_domain::account::AccountStatus;
use thiserror::Error;

/// Reports a domain status that cannot be materialized by an account fragment.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum MaterializedAccountStatusError {
    #[error("account status {0:?} cannot be materialized as an account fragment status")]
    Unsupported(AccountStatus),
}
