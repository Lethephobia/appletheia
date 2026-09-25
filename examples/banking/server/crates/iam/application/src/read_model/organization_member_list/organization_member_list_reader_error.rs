use thiserror::Error;

/// Errors returned while reading organization member lists.
#[derive(Debug, Error)]
pub enum OrganizationMemberListReaderError {
    #[error("organization member list persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
