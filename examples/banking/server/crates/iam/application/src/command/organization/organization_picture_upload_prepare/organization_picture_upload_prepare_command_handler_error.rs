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

    #[error("organization was not found")]
    OrganizationNotFound,

    #[error("removed organizations cannot prepare picture uploads")]
    OrganizationRemoved,

    #[error("picture content length exceeds the configured maximum")]
    ContentLengthTooLarge,

    #[error("picture content type is not allowed")]
    ContentTypeNotAllowed,
}
