use crate::onchain::{OnchainAccountAddress, TokenProgramId};

/// Configuration for `CurrencyMintAccountCreateCommandHandler`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurrencyMintAccountCreateCommandHandlerConfig {
    token_program_id: TokenProgramId,
    mint_authority: OnchainAccountAddress,
    freeze_authority: Option<OnchainAccountAddress>,
}

impl CurrencyMintAccountCreateCommandHandlerConfig {
    pub fn new(
        token_program_id: TokenProgramId,
        mint_authority: OnchainAccountAddress,
        freeze_authority: Option<OnchainAccountAddress>,
    ) -> Self {
        Self {
            token_program_id,
            mint_authority,
            freeze_authority,
        }
    }

    pub fn token_program_id(&self) -> &TokenProgramId {
        &self.token_program_id
    }

    pub fn mint_authority(&self) -> &OnchainAccountAddress {
        &self.mint_authority
    }

    pub fn freeze_authority(&self) -> Option<&OnchainAccountAddress> {
        self.freeze_authority.as_ref()
    }
}
