use thiserror::Error;

use crate::request_context::ActorRefError;

/// Represents errors returned while constructing a request context.
#[derive(Debug, Error)]
pub enum RequestContextError {
    #[error("actor reference could not be derived from principal")]
    ActorRef(#[from] ActorRefError),
}
