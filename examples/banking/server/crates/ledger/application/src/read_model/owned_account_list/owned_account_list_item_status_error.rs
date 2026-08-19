use banking_ledger_domain::account::AccountStatus;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum OwnedAccountListItemStatusError {
    #[error("account status {0:?} cannot be projected into an owned account list status")]
    Unsupported(AccountStatus),
}
