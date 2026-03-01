use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthTokenIssueError {
    #[error("token issue failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}
