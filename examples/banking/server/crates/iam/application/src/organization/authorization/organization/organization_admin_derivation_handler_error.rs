use appletheia::application::aggregate::SerializedAggregateError;
use banking_iam_domain::OrganizationRoleAssignmentError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationAdminDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    OrganizationRoleAssignment(#[from] OrganizationRoleAssignmentError),
}
