use appletheia::application::aggregate::SerializedAggregateError;
use banking_iam_domain::OrganizationRoleAssignmentError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationRoleAssignmentOrganizationDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    RoleAssignment(#[from] OrganizationRoleAssignmentError),
}
