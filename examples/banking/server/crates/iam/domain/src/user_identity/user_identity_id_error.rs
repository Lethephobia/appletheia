use thiserror::Error;
use uuid::Uuid;

/// Describes why a user-identity ID is invalid.
#[derive(Debug, Error)]
pub enum UserIdentityIdError {
    #[error("user identity id must be UUID v5: {0}")]
    NotUuidV5(Uuid),
}
