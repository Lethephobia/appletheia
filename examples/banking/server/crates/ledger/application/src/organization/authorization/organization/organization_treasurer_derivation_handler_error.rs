use appletheia::application::aggregate::SerializedAggregateError;
use banking_ledger_domain::OrganizationLedgerRoleAssignmentError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationTreasurerDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    RoleAssignment(#[from] OrganizationLedgerRoleAssignmentError),
}
