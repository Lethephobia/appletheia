use banking_iam_domain::UserStatus;
use thiserror::Error;

/// Reports a domain status that cannot be materialized by the public user fragment.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum MaterializedUserStatusError {
    #[error("user status {0:?} cannot be materialized as a public user fragment status")]
    Unsupported(UserStatus),
}
