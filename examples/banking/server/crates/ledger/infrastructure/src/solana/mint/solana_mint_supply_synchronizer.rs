use anchor_lang::{InstructionData, ToAccountMetas};
use banking_ledger::{
    BankingLedgerConfig, Mint, MintState, ProgramAuthority,
    accounts::MintSupplySyncInstructionAccounts, instruction::SyncMintSupply,
};
use banking_ledger_application::mint::{
    MintSupplySyncRequest, MintSupplySynchronizer, MintSupplySynchronizerError,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Signer};
use solana_transaction::Transaction;
use spl_associated_token_account_interface::address as associated_token_address;

use super::{
    BankingLedgerMintId, SolanaMintSupplySynchronizerConfig, SolanaMintSupplySynchronizerError,
};

/// Solana implementation of `MintSupplySynchronizer`.
pub struct SolanaMintSupplySynchronizer {
    rpc_client: RpcClient,
    config: SolanaMintSupplySynchronizerConfig,
}

impl SolanaMintSupplySynchronizer {
    pub fn new(rpc_client: RpcClient, config: SolanaMintSupplySynchronizerConfig) -> Self {
        Self { rpc_client, config }
    }

    fn banking_ledger_config_address(&self) -> Pubkey {
        Pubkey::find_program_address(&[BankingLedgerConfig::SEED], self.config.program_id()).0
    }

    fn mint_state_address(&self, mint_id: &[u8; 16]) -> Pubkey {
        Pubkey::find_program_address(&[MintState::SEED, mint_id], self.config.program_id()).0
    }

    fn mint_address(&self, mint_id: &[u8; 16]) -> Pubkey {
        Pubkey::find_program_address(&[Mint::SEED, mint_id], self.config.program_id()).0
    }

    fn program_authority_address(&self) -> Pubkey {
        Pubkey::find_program_address(&[ProgramAuthority::SEED], self.config.program_id()).0
    }

    fn pool_token_account_address(
        program_authority_address: &Pubkey,
        mint_address: &Pubkey,
        token_program_id: &Pubkey,
    ) -> Pubkey {
        associated_token_address::get_associated_token_address_with_program_id(
            program_authority_address,
            mint_address,
            token_program_id,
        )
    }

    async fn send_transaction(
        &self,
        instructions: Vec<Instruction>,
        signers: Vec<&dyn Signer>,
    ) -> Result<(), SolanaMintSupplySynchronizerError> {
        let blockhash = self.rpc_client.get_latest_blockhash().await?;
        let payer = self.config.payer().as_ref();
        let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
        transaction.try_sign(&signers, blockhash)?;

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

impl MintSupplySynchronizer for SolanaMintSupplySynchronizer {
    async fn sync_supply(
        &self,
        request: MintSupplySyncRequest,
    ) -> Result<(), MintSupplySynchronizerError> {
        let token_program_id = spl_token_2022_interface::id();
        let associated_token_program_id = spl_associated_token_account_interface::program::id();
        let program_id = *self.config.program_id();
        let operator = self.config.operator().pubkey();
        let mint_id = BankingLedgerMintId::from(request.currency_id()).into_bytes();
        let banking_ledger_config_address = self.banking_ledger_config_address();
        let mint_state_address = self.mint_state_address(&mint_id);
        let program_authority_address = self.program_authority_address();
        let mint_address = self.mint_address(&mint_id);
        let pool_token_account_address = Self::pool_token_account_address(
            &program_authority_address,
            &mint_address,
            &token_program_id,
        );
        let target_supply = u64::try_from(request.target_supply().value()).map_err(|_| {
            MintSupplySynchronizerError::Backend(Box::new(
                SolanaMintSupplySynchronizerError::TargetSupplyOverflow,
            ))
        })?;

        let instruction = Instruction {
            program_id,
            accounts: MintSupplySyncInstructionAccounts {
                banking_ledger_config: banking_ledger_config_address,
                operator,
                mint_state: mint_state_address,
                program_authority: program_authority_address,
                mint: mint_address,
                pool_token_account: pool_token_account_address,
                token_program: token_program_id,
                associated_token_program: associated_token_program_id,
            }
            .to_account_metas(None),
            data: SyncMintSupply {
                mint_id,
                target_supply,
            }
            .data(),
        };
        let payer = self.config.payer().as_ref();
        let operator = self.config.operator().as_ref();
        let signers = self.unique_signers(&[payer, operator]);

        self.send_transaction(vec![instruction], signers)
            .await
            .map_err(|error| MintSupplySynchronizerError::Backend(Box::new(error)))
    }
}
