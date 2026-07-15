use banking_iam_domain::UserStatus;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum UserPublicProfileStatusError {
    #[error("user status {0:?} cannot be projected into a public profile status")]
    Unsupported(UserStatus),
}
