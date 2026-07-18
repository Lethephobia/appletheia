use appletheia::application::Retryability;

use appletheia::application::object_storage::{ObjectNameError, ObjectUploadSignerError};
use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{Organization, OrganizationError, OrganizationPictureObjectNameError};
use thiserror::Error;

/// Represents errors returned while preparing an organization-picture upload.
#[derive(Debug, Error)]
pub enum OrganizationPictureUploadPrepareCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("picture object name generation failed")]
    PictureObjectName(#[from] OrganizationPictureObjectNameError),

    #[error("object storage object name generation failed")]
    ObjectName(#[from] ObjectNameError),

    #[error("object upload signer failed")]
    ObjectUploadSigner(#[from] ObjectUploadSignerError),
}

impl Retryability for OrganizationPictureUploadPrepareCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::OrganizationRepository(error) => error.is_retryable(),
            Self::Organization(_) => false,
            Self::PictureObjectName(_) => false,
            Self::ObjectName(_) => false,
            Self::ObjectUploadSigner(error) => error.is_retryable(),
        }
    }
}
