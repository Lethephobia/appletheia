use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
pub enum OidcBirthMonthError {
    #[error("birth month is out of range: {value}")]
    OutOfRange { value: u8 },
}
