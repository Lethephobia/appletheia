use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
pub enum OidcBirthdateFullError {
    #[error("invalid birthdate full date")]
    InvalidDate,
}
