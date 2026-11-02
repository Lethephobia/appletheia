use appletheia::application::aggregate::SerializedAggregateError;
use banking_iam_domain::TeamRoleAssignmentError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamRoleAssignmentTeamDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    RoleAssignment(#[from] TeamRoleAssignmentError),
}
