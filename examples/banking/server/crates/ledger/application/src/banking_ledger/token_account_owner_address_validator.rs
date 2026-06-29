use super::{TokenAccountOwnerAddress, TokenAccountOwnerAddressValidatorError};

/// Validates token account owner addresses against the active token rail.
#[allow(async_fn_in_trait)]
pub trait TokenAccountOwnerAddressValidator: Send + Sync {
    async fn validate(
        &self,
        address: &TokenAccountOwnerAddress,
    ) -> Result<(), TokenAccountOwnerAddressValidatorError>;
}
