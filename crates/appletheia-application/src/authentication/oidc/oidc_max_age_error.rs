use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
pub enum OidcMaxAgeError {
    #[error("max_age must not be negative")]
    Negative,
}
