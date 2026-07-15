use banking_ledger_domain::core::TokenAccountOwnerAddress;

use super::TokenAccountOwnerAddressValidatorError;

#[allow(async_fn_in_trait)]
pub trait TokenAccountOwnerAddressValidator: Send + Sync {
    async fn validate(
        &self,
        address: &TokenAccountOwnerAddress,
    ) -> Result<(), TokenAccountOwnerAddressValidatorError>;
}
