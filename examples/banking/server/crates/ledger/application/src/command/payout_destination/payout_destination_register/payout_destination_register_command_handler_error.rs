use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::payout_destination::{PayoutDestination, PayoutDestinationError};
use thiserror::Error;

use crate::mint::{TokenAccountOwnerAddressError, TokenAccountOwnerAddressValidatorError};

/// Represents errors returned while registering a payout destination.
#[derive(Debug, Error)]
pub enum PayoutDestinationRegisterCommandHandlerError {
    #[error("payout destination repository failed")]
    PayoutDestinationRepository(#[from] RepositoryError<PayoutDestination>),

    #[error("payout destination aggregate failed")]
    PayoutDestination(#[from] PayoutDestinationError),

    #[error("token account owner address validation failed")]
    TokenAccountOwnerAddressValidator(#[from] TokenAccountOwnerAddressValidatorError),

    #[error("token account owner address is invalid")]
    TokenAccountOwnerAddress(#[from] TokenAccountOwnerAddressError),
}
