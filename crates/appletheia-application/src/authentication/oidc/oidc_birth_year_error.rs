use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
pub enum OidcBirthYearError {
    #[error("invalid birth year format")]
    InvalidFormat,
}
