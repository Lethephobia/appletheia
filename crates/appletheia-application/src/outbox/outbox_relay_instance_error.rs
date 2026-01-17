use thiserror::Error;

#[derive(Debug, Error)]
pub enum OutboxRelayInstanceError {
    #[error("outbox relay instance must contain ':' separator")]
    MissingSeparator,
    #[error("outbox relay instance id must not be empty")]
    EmptyInstanceId,
    #[error("outbox relay process id must not be empty")]
    EmptyProcessId,
    #[error("invalid outbox relay process id")]
    InvalidProcessId(#[source] std::num::ParseIntError),
}
