use thiserror::Error;

/// Errors returned while writing organization member lists.
#[derive(Debug, Error)]
pub enum OrganizationMemberListWriterError {
    #[error("organization member list persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
