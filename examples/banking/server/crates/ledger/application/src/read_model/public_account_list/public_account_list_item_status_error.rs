use banking_ledger_domain::account::AccountStatus;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum PublicAccountListItemStatusError {
    #[error("account status {0:?} cannot be projected into a public account list status")]
    Unsupported(AccountStatus),
}
