use thiserror::Error;

/// Represents errors returned while converting principals into actor references.
#[derive(Debug, Error)]
pub enum ActorRefError {
    #[error("principal is unavailable and cannot be converted to an actor reference")]
    PrincipalUnavailable,
}
