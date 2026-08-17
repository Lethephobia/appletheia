use banking_iam_domain::UserId;
use thiserror::Error;

/// Reports failure to materialize a complete fragment dependency graph.
#[derive(Debug, Error)]
pub enum PgFragmentLoaderError {
    #[error("fragment dependency persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("fragment dependency user was not found: {user_id}")]
    UserNotFound { user_id: UserId },
}
