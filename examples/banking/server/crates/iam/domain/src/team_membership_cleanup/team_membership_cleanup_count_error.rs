use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamMembershipCleanupCountError {
    #[error("team membership cleanup count exceeds the supported range")]
    Overflow,
}
