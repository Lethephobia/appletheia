use std::str::FromStr;

use banking_ledger_application::mint::{
    TokenAccountOwnerAddressValidationResult, TokenAccountOwnerAddressValidator,
    TokenAccountOwnerAddressValidatorError,
};
use banking_ledger_domain::core::TokenAccountOwnerAddress;
use solana_sdk::pubkey::Pubkey;

/// Validates token account owner addresses as Solana public keys.
#[derive(Clone, Copy, Debug, Default)]
pub struct SolanaTokenAccountOwnerAddressValidator;

impl TokenAccountOwnerAddressValidator for SolanaTokenAccountOwnerAddressValidator {
    async fn validate(
        &self,
        address: &TokenAccountOwnerAddress,
    ) -> Result<TokenAccountOwnerAddressValidationResult, TokenAccountOwnerAddressValidatorError>
    {
        Ok(match Pubkey::from_str(address.value()) {
            Ok(_) => TokenAccountOwnerAddressValidationResult::Valid,
            Err(_) => TokenAccountOwnerAddressValidationResult::Invalid,
        })
    }
}
