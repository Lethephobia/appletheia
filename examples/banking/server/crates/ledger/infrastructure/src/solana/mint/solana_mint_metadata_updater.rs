use anchor_lang::{InstructionData, ToAccountMetas};
use appletheia::domain::AggregateId;
use banking_ledger::{
    BankingLedgerConfig, MintMetadata, MintState, ProgramAuthority,
    accounts::MintMetadataUpdateInstructionAccounts, instruction::UpdateMintMetadata,
};
use banking_ledger_application::mint::{
    MintMetadataUpdateRequest, MintMetadataUpdater, MintMetadataUpdaterError,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Signer};
use solana_transaction::Transaction;

use super::{
    MintMetadataPublishRequest, MintMetadataPublisher, SolanaMintMetadataUpdaterConfig,
    SolanaMintMetadataUpdaterError,
};

/// Solana implementation of `MintMetadataUpdater`.
pub struct SolanaMintMetadataUpdater<P>
where
    P: MintMetadataPublisher,
{
    rpc_client: RpcClient,
    mint_metadata_publisher: P,
    config: SolanaMintMetadataUpdaterConfig,
}

impl<P> SolanaMintMetadataUpdater<P>
where
    P: MintMetadataPublisher,
{
    pub fn new(
        rpc_client: RpcClient,
        mint_metadata_publisher: P,
        config: SolanaMintMetadataUpdaterConfig,
    ) -> Self {
        Self {
            rpc_client,
            mint_metadata_publisher,
            config,
        }
    }

    fn banking_ledger_config_address(&self) -> Pubkey {
        Pubkey::find_program_address(&[BankingLedgerConfig::SEED], self.config.program_id()).0
    }

    fn mint_state_address(&self, mint_id: &[u8; 16]) -> Pubkey {
        Pubkey::find_program_address(&[MintState::SEED, mint_id], self.config.program_id()).0
    }

    fn mint_metadata_address(&self, mint_id: &[u8; 16]) -> Pubkey {
        Pubkey::find_program_address(&[MintMetadata::SEED, mint_id], self.config.program_id()).0
    }

    fn program_authority_address(&self) -> Pubkey {
        Pubkey::find_program_address(&[ProgramAuthority::SEED], self.config.program_id()).0
    }

    async fn publish_metadata(
        &self,
        request: &MintMetadataUpdateRequest,
    ) -> Result<String, MintMetadataUpdaterError> {
        self.mint_metadata_publisher
            .publish(MintMetadataPublishRequest::new(
                request.currency_id(),
                request.name().clone(),
                request.symbol().clone(),
                request.description().cloned(),
                request.image().cloned(),
            ))
            .await
            .map_err(|error| MintMetadataUpdaterError::Backend(Box::new(error)))
    }

    async fn send_transaction(
        &self,
        instructions: Vec<Instruction>,
        signers: Vec<&dyn Signer>,
    ) -> Result<(), SolanaMintMetadataUpdaterError> {
        let blockhash = self.rpc_client.get_latest_blockhash().await?;
        let transaction = {
            let payer = self.config.payer().as_ref();
            let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
            transaction.try_sign(&signers, blockhash)?;
            transaction
        };

        self.rpc_client
            .send_and_confirm_transaction(&transaction)
            .await?;

        Ok(())
    }

    fn unique_signers<'a>(&self, signers: &[&'a dyn Signer]) -> Vec<&'a dyn Signer> {
        let mut unique: Vec<&dyn Signer> = Vec::new();
        for signer in signers {
            if unique
                .iter()
                .all(|existing| existing.pubkey() != signer.pubkey())
            {
                unique.push(*signer);
            }
        }

        unique
    }
}

impl<P> MintMetadataUpdater for SolanaMintMetadataUpdater<P>
where
    P: MintMetadataPublisher,
{
    async fn update(
        &self,
        request: MintMetadataUpdateRequest,
    ) -> Result<(), MintMetadataUpdaterError> {
        let token_program_id = spl_token_2022_interface::id();
        let program_id = *self.config.program_id();
        let operator = self.config.operator().pubkey();
        let mint_id = *request.currency_id().value().as_bytes();
        let metadata_uri = self.publish_metadata(&request).await?;
        let banking_ledger_config_address = self.banking_ledger_config_address();
        let mint_state_address = self.mint_state_address(&mint_id);
        let program_authority_address = self.program_authority_address();
        let mint_metadata_address = self.mint_metadata_address(&mint_id);
        let instruction = Instruction {
            program_id,
            accounts: MintMetadataUpdateInstructionAccounts {
                banking_ledger_config: banking_ledger_config_address,
                operator,
                mint_state: mint_state_address,
                program_authority: program_authority_address,
                mint_metadata: mint_metadata_address,
                token_program: token_program_id,
            }
            .to_account_metas(None),
            data: UpdateMintMetadata {
                mint_id,
                name: request.name().value().to_owned(),
                symbol: request.symbol().value().to_owned(),
                uri: metadata_uri,
            }
            .data(),
        };
        let payer = self.config.payer().as_ref();
        let operator = self.config.operator().as_ref();
        let signers = self.unique_signers(&[payer, operator]);

        self.send_transaction(vec![instruction], signers)
            .await
            .map_err(|error| MintMetadataUpdaterError::Backend(Box::new(error)))
    }
}
