use anchor_lang::{InstructionData, ToAccountMetas};
use banking_ledger::{
    BankingLedgerConfig, Mint, MintMetadata, MintState, ProgramAuthority,
    accounts::{MintUpsertInstructionAccounts, PoolTokenAccountEnsureInstructionAccounts},
    instruction::{EnsurePoolTokenAccount, UpsertMint},
};
use banking_ledger_application::{
    MintAccountAddress, MintProvisionReceipt, MintProvisionRequest, MintProvisioner,
    MintProvisionerError, PoolTokenAccountAddress,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Signer};
use solana_system_interface::program as system_program;
use solana_transaction::Transaction;
use spl_associated_token_account_interface::address as associated_token_address;

use super::{SolanaMintProvisionerConfig, SolanaMintProvisionerError};

/// Solana implementation of `MintProvisioner`.
pub struct SolanaMintProvisioner {
    rpc_client: RpcClient,
    config: SolanaMintProvisionerConfig,
}

impl SolanaMintProvisioner {
    pub fn new(rpc_client: RpcClient, config: SolanaMintProvisionerConfig) -> Self {
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
    ) -> Result<(), SolanaMintProvisionerError> {
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

    fn receipt(
        mint_address: Pubkey,
        pool_token_account_address: Pubkey,
    ) -> Result<MintProvisionReceipt, MintProvisionerError> {
        Ok(MintProvisionReceipt::new(
            MintAccountAddress::try_from(mint_address.to_string()).map_err(|error| {
                MintProvisionerError::Backend(Box::new(
                    SolanaMintProvisionerError::MintAccountAddress(error),
                ))
            })?,
            PoolTokenAccountAddress::try_from(pool_token_account_address.to_string()).map_err(
                |error| {
                    MintProvisionerError::Backend(Box::new(
                        SolanaMintProvisionerError::PoolTokenAccountAddress(error),
                    ))
                },
            )?,
        ))
    }
}

impl MintProvisioner for SolanaMintProvisioner {
    async fn provision(
        &self,
        request: MintProvisionRequest,
    ) -> Result<MintProvisionReceipt, MintProvisionerError> {
        let token_program_id = spl_token_2022_interface::id();
        let associated_token_program_id = spl_associated_token_account_interface::program::id();
        let program_id = *self.config.program_id();
        let operator = self.config.operator().pubkey();
        let mint_id = request.mint_id().bytes();
        let banking_ledger_config_address = Self::banking_ledger_config_address(&program_id);
        let mint_state_address = Self::mint_state_address(&mint_id, &program_id);
        let program_authority_address = Self::program_authority_address(&program_id);
        let mint_address = Self::mint_address(&mint_id, &program_id);
        let mint_metadata_address = Self::mint_metadata_address(&mint_id, &program_id);
        let pool_token_account_address = Self::pool_token_account_address(
            &program_authority_address,
            &mint_address,
            &token_program_id,
        );
        let upsert_mint_instruction = Instruction {
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
                decimals: request.decimals().value(),
                name: request.metadata().name().value().to_owned(),
                symbol: request.metadata().symbol().value().to_owned(),
                uri: request.metadata().uri().to_string(),
            }
            .data(),
        };
        let ensure_pool_token_account_instruction = Instruction {
            program_id,
            accounts: PoolTokenAccountEnsureInstructionAccounts {
                payer: self.config.payer().pubkey(),
                banking_ledger_config: banking_ledger_config_address,
                operator,
                mint_state: mint_state_address,
                program_authority: program_authority_address,
                mint: mint_address,
                pool_token_account: pool_token_account_address,
                system_program: system_program::id(),
                token_program: token_program_id,
                associated_token_program: associated_token_program_id,
            }
            .to_account_metas(None),
            data: EnsurePoolTokenAccount { mint_id }.data(),
        };
        let payer = self.config.payer().as_ref();
        let operator = self.config.operator().as_ref();
        let signers = unique_signers(&[payer, operator]);

        self.send_transaction(
            vec![
                upsert_mint_instruction,
                ensure_pool_token_account_instruction,
            ],
            signers,
        )
        .await
        .map_err(|error| MintProvisionerError::Backend(Box::new(error)))?;

        Self::receipt(mint_address, pool_token_account_address)
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
