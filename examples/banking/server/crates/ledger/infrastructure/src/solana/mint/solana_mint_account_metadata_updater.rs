use banking_ledger_application::{
    MintAccountMetadataUpdateRequest, MintAccountMetadataUpdater, MintAccountMetadataUpdaterError,
    MintAccountSeed,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction, pubkey::Pubkey, pubkey::PubkeyError, signature::Signer,
};
use solana_transaction::Transaction;
use spl_token_metadata_interface::instruction as token_metadata_instruction;
use spl_token_metadata_interface::state::Field;

use super::{SolanaMintAccountMetadataUpdaterConfig, SolanaMintAccountMetadataUpdaterError};

/// Solana implementation of `MintAccountMetadataUpdater`.
pub struct SolanaMintAccountMetadataUpdater {
    rpc_client: RpcClient,
    config: SolanaMintAccountMetadataUpdaterConfig,
}

impl SolanaMintAccountMetadataUpdater {
    pub fn new(rpc_client: RpcClient, config: SolanaMintAccountMetadataUpdaterConfig) -> Self {
        Self { rpc_client, config }
    }

    fn mint_address(
        mint_authority: &Pubkey,
        seed: &MintAccountSeed,
        token_program_id: &Pubkey,
    ) -> Result<Pubkey, PubkeyError> {
        Pubkey::create_with_seed(mint_authority, seed.value(), token_program_id)
    }

    async fn send_transaction(
        &self,
        instructions: Vec<Instruction>,
    ) -> Result<(), SolanaMintAccountMetadataUpdaterError> {
        let blockhash = self.rpc_client.get_latest_blockhash().await?;
        let transaction = {
            let payer = self.config.payer().as_ref();
            let mint_authority = self.config.mint_authority().as_ref();
            let signers: Vec<&dyn Signer> = if payer.pubkey() == mint_authority.pubkey() {
                vec![payer]
            } else {
                vec![payer, mint_authority]
            };
            let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
            transaction.try_sign(&signers, blockhash)?;
            transaction
        };

        self.rpc_client
            .send_and_confirm_transaction(&transaction)
            .await?;

        Ok(())
    }
}

impl MintAccountMetadataUpdater for SolanaMintAccountMetadataUpdater {
    async fn update(
        &self,
        request: MintAccountMetadataUpdateRequest,
    ) -> Result<(), MintAccountMetadataUpdaterError> {
        let token_program_id = spl_token_2022_interface::id();
        let mint_authority = self.config.mint_authority().pubkey();
        let mint_address = Self::mint_address(&mint_authority, request.seed(), &token_program_id)
            .map_err(|error| {
            MintAccountMetadataUpdaterError::Backend(Box::new(
                SolanaMintAccountMetadataUpdaterError::MintAccountAddressDerivation(error),
            ))
        })?;
        let instructions = vec![
            token_metadata_instruction::update_field(
                &token_program_id,
                &mint_address,
                &mint_authority,
                Field::Name,
                request.metadata().name().value().to_owned(),
            ),
            token_metadata_instruction::update_field(
                &token_program_id,
                &mint_address,
                &mint_authority,
                Field::Symbol,
                request.metadata().symbol().value().to_owned(),
            ),
            token_metadata_instruction::update_field(
                &token_program_id,
                &mint_address,
                &mint_authority,
                Field::Uri,
                request.metadata().uri().to_string(),
            ),
        ];

        self.send_transaction(instructions)
            .await
            .map_err(|error| MintAccountMetadataUpdaterError::Backend(Box::new(error)))
    }
}
