use std::str::FromStr;

use banking_ledger_application::{
    SolanaTokenBindingSettlementValidationRequest, SolanaTokenBindingSettlementValidator,
    TokenBindingSettlementValidatorError,
};
use banking_ledger_domain::core::TokenDecimals;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use spl_token_2022_interface::{extension::StateWithExtensions, state::Mint};

pub struct DefaultSolanaTokenBindingSettlementValidator {
    rpc_client: RpcClient,
}

impl DefaultSolanaTokenBindingSettlementValidator {
    pub fn new(rpc_client: RpcClient) -> Self {
        Self { rpc_client }
    }

    fn supported_token_program(owner: Pubkey) -> bool {
        let legacy = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
        let token_2022 = Pubkey::from_str("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");
        legacy.is_ok_and(|program| owner == program)
            || token_2022.is_ok_and(|program| owner == program)
    }
}

impl SolanaTokenBindingSettlementValidator for DefaultSolanaTokenBindingSettlementValidator {
    async fn validate(
        &self,
        request: SolanaTokenBindingSettlementValidationRequest,
    ) -> Result<(), TokenBindingSettlementValidatorError> {
        let mint = Pubkey::new_from_array(*request.token_address().address().as_bytes());
        let account = self
            .rpc_client
            .get_account(&mint)
            .await
            .map_err(|error| TokenBindingSettlementValidatorError::Backend(Box::new(error)))?;
        if !Self::supported_token_program(account.owner) {
            return Err(TokenBindingSettlementValidatorError::Incompatible);
        }
        let mint_state = StateWithExtensions::<Mint>::unpack(&account.data)
            .map_err(|_| TokenBindingSettlementValidatorError::Incompatible)?;
        let token_decimals = TokenDecimals::new(mint_state.base.decimals);
        if token_decimals.value() < request.currency_decimals().value() {
            return Err(TokenBindingSettlementValidatorError::Incompatible);
        }
        Ok(())
    }
}
