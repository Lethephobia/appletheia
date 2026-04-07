use appletheia::application::repository::RepositoryError;
use appletheia::domain::AggregateId;
use banking_iam_domain::{Organization, OrganizationError, UserId};
use thiserror::Error;

/// Represents errors returned while creating an organization.
#[derive(Debug, Error)]
pub enum OrganizationCreateCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("organization id is missing after create")]
    MissingOrganizationId,

    #[error("organization owner principal contains an invalid user id")]
    InvalidOwnerUserId(#[source] <UserId as AggregateId>::Error),
}
