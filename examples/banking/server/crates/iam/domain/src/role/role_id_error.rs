use thiserror::Error;

/// Describes why a role ID is invalid.
#[derive(Debug, Error)]
pub enum RoleIdError {
    #[error("role id must be a uuid v5")]
    NotUuidV5,
}
