use std::str::FromStr;

use banking_ledger_application::mint::{
    TokenAccountOwnerAddressValidator, TokenAccountOwnerAddressValidatorError,
};
use banking_ledger_domain::payout_destination::TokenAccountOwnerAddress;
use solana_sdk::pubkey::Pubkey;

/// Validates token account owner addresses as Solana public keys.
#[derive(Clone, Copy, Debug, Default)]
pub struct SolanaTokenAccountOwnerAddressValidator;

impl TokenAccountOwnerAddressValidator for SolanaTokenAccountOwnerAddressValidator {
    async fn validate(
        &self,
        address: &TokenAccountOwnerAddress,
    ) -> Result<(), TokenAccountOwnerAddressValidatorError> {
        Pubkey::from_str(address.value())
            .map_err(|_| TokenAccountOwnerAddressValidatorError::InvalidAddress)?;

        Ok(())
    }
}
