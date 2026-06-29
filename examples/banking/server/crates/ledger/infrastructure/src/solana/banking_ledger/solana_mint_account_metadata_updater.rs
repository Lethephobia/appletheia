use anchor_lang::{InstructionData, ToAccountMetas};
use banking_ledger::{
    BankingLedgerConfig, Mint, MintMetadata, MintState, ProgramAuthority,
    accounts::MintUpsertInstructionAccounts, instruction::UpsertMint,
};
use banking_ledger_application::{
    MintAccountMetadataUpdateRequest, MintAccountMetadataUpdater, MintAccountMetadataUpdaterError,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Signer};
use solana_system_interface::program as system_program;
use solana_transaction::Transaction;
use spl_token_2022_interface::{extension::StateWithExtensions, state::Mint as TokenMint};

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

    fn banking_ledger_config_address(program_id: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(&[BankingLedgerConfig::SEED], program_id).0
    }

    fn mint_state_address(mint_id: &[u8; 16], program_id: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(&[MintState::SEED, mint_id], program_id).0
    }

    fn mint_address(mint_id: &[u8; 16], program_id: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(&[Mint::SEED, mint_id], program_id).0
    }

    fn mint_metadata_address(mint_id: &[u8; 16], program_id: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(&[MintMetadata::SEED, mint_id], program_id).0
    }

    fn program_authority_address(program_id: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(&[ProgramAuthority::SEED], program_id).0
    }

    async fn send_transaction(
        &self,
        instructions: Vec<Instruction>,
        signers: Vec<&dyn Signer>,
    ) -> Result<(), SolanaMintAccountMetadataUpdaterError> {
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

    async fn mint_decimals(
        &self,
        mint_address: &Pubkey,
    ) -> Result<u8, SolanaMintAccountMetadataUpdaterError> {
        let account = self.rpc_client.get_account(mint_address).await?;
        let mint = StateWithExtensions::<TokenMint>::unpack(&account.data)
            .map_err(SolanaMintAccountMetadataUpdaterError::MintAccountData)?;

        Ok(mint.base.decimals)
    }
}

impl MintAccountMetadataUpdater for SolanaMintAccountMetadataUpdater {
    async fn update(
        &self,
        request: MintAccountMetadataUpdateRequest,
    ) -> Result<(), MintAccountMetadataUpdaterError> {
        let token_program_id = spl_token_2022_interface::id();
        let program_id = *self.config.program_id();
        let operator = self.config.operator().pubkey();
        let mint_id = request.mint_id().bytes();
        let banking_ledger_config_address = Self::banking_ledger_config_address(&program_id);
        let mint_state_address = Self::mint_state_address(&mint_id, &program_id);
        let program_authority_address = Self::program_authority_address(&program_id);
        let mint_address = Self::mint_address(&mint_id, &program_id);
        let mint_metadata_address = Self::mint_metadata_address(&mint_id, &program_id);
        let decimals = self
            .mint_decimals(&mint_address)
            .await
            .map_err(|error| MintAccountMetadataUpdaterError::Backend(Box::new(error)))?;
        let instruction = Instruction {
            program_id,
            accounts: MintUpsertInstructionAccounts {
                payer: self.config.payer().pubkey(),
                banking_ledger_config: banking_ledger_config_address,
                operator,
                mint_state: mint_state_address,
                program_authority: program_authority_address,
                mint: mint_address,
                mint_metadata: mint_metadata_address,
                system_program: system_program::id(),
                token_program: token_program_id,
            }
            .to_account_metas(None),
            data: UpsertMint {
                mint_id,
                decimals,
                name: request.metadata().name().value().to_owned(),
                symbol: request.metadata().symbol().value().to_owned(),
                uri: request.metadata().uri().to_string(),
            }
            .data(),
        };
        let payer = self.config.payer().as_ref();
        let operator = self.config.operator().as_ref();
        let signers = unique_signers(&[payer, operator]);

        self.send_transaction(vec![instruction], signers)
            .await
            .map_err(|error| MintAccountMetadataUpdaterError::Backend(Box::new(error)))
    }
}

fn unique_signers<'a>(signers: &[&'a dyn Signer]) -> Vec<&'a dyn Signer> {
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
