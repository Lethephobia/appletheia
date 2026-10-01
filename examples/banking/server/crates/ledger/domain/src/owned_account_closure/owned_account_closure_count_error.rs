use thiserror::Error;

#[derive(Debug, Error)]
pub enum OwnedAccountClosureCountError {
    #[error("owned account closure count exceeds the supported range")]
    Overflow,
}
