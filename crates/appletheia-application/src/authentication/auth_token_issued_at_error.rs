use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum AuthTokenIssuedAtError {
    #[error("issued_at must not be before the unix epoch")]
    BeforeUnixEpoch,

    #[error("invalid unix timestamp seconds: {seconds}")]
    InvalidUnixTimestampSeconds { seconds: u64 },
}
