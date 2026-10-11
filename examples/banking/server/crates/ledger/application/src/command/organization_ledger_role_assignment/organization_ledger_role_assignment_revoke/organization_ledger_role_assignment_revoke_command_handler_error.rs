use appletheia::application::{Retryability, repository::RepositoryError};
use banking_ledger_domain::{
    OrganizationLedgerRoleAssignment, OrganizationLedgerRoleAssignmentError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationLedgerRoleAssignmentRevokeCommandHandlerError {
    #[error(transparent)]
    OrganizationLedgerRoleAssignmentRepository(
        #[from] RepositoryError<OrganizationLedgerRoleAssignment>,
    ),

    #[error(transparent)]
    OrganizationLedgerRoleAssignment(#[from] OrganizationLedgerRoleAssignmentError),
}

impl Retryability for OrganizationLedgerRoleAssignmentRevokeCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::OrganizationLedgerRoleAssignmentRepository(error) => error.is_retryable(),
            Self::OrganizationLedgerRoleAssignment(_) => false,
        }
    }
}
