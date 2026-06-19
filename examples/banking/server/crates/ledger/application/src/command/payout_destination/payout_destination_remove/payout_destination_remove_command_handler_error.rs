use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::payout_destination::{PayoutDestination, PayoutDestinationError};
use thiserror::Error;

/// Represents errors returned while removing a payout destination.
#[derive(Debug, Error)]
pub enum PayoutDestinationRemoveCommandHandlerError {
    #[error("payout destination repository failed")]
    PayoutDestinationRepository(#[from] RepositoryError<PayoutDestination>),

    #[error("payout destination aggregate failed")]
    PayoutDestination(#[from] PayoutDestinationError),
}
