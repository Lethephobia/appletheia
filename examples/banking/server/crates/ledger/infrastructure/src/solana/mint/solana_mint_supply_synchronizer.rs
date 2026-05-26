use banking_ledger_application::{
    MintAccountSeed, MintSupplySyncRequest, MintSupplySynchronizer, MintSupplySynchronizerError,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction, program_error::ProgramError, pubkey::Pubkey, pubkey::PubkeyError,
    signature::Signer,
};
use solana_transaction::Transaction;
use spl_associated_token_account_interface::address as associated_token_address;
use spl_token_2022_interface::extension::PodStateWithExtensions;
use spl_token_2022_interface::instruction as token_instruction;
use spl_token_2022_interface::pod::PodMint;

use super::{SolanaMintSupplySynchronizerConfig, SolanaMintSupplySynchronizerError};

/// Solana implementation of `MintSupplySynchronizer`.
pub struct SolanaMintSupplySynchronizer {
    rpc_client: RpcClient,
    config: SolanaMintSupplySynchronizerConfig,
}

impl SolanaMintSupplySynchronizer {
    pub fn new(rpc_client: RpcClient, config: SolanaMintSupplySynchronizerConfig) -> Self {
        Self { rpc_client, config }
    }

    fn mint_address(
        mint_authority: &Pubkey,
        seed: &MintAccountSeed,
        token_program_id: &Pubkey,
    ) -> Result<Pubkey, PubkeyError> {
        Pubkey::create_with_seed(mint_authority, seed.value(), token_program_id)
    }

    fn pool_token_account_address(
        pool_account_owner: &Pubkey,
        mint_address: &Pubkey,
        token_program_id: &Pubkey,
    ) -> Pubkey {
        associated_token_address::get_associated_token_address_with_program_id(
            pool_account_owner,
            mint_address,
            token_program_id,
        )
    }

    async fn current_supply(
        &self,
        mint_address: &Pubkey,
        token_program_id: &Pubkey,
    ) -> Result<u64, SolanaMintSupplySynchronizerError> {
        let account = self
            .rpc_client
            .get_account_with_commitment(mint_address, self.rpc_client.commitment())
            .await?
            .value
            .ok_or_else(
                || SolanaMintSupplySynchronizerError::MintAccountInvalidData {
                    address: mint_address.to_string(),
                    source: ProgramError::UninitializedAccount,
                },
            )?;

        if account.owner != *token_program_id {
            return Err(
                SolanaMintSupplySynchronizerError::MintAccountUnexpectedOwner {
                    address: mint_address.to_string(),
                    owner: account.owner.to_string(),
                    expected_owner: token_program_id.to_string(),
                },
            );
        }

        let state = PodStateWithExtensions::<PodMint>::unpack(&account.data).map_err(|source| {
            SolanaMintSupplySynchronizerError::MintAccountInvalidData {
                address: mint_address.to_string(),
                source,
            }
        })?;

        Ok(u64::from(state.base.supply))
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
}

impl MintSupplySynchronizer for SolanaMintSupplySynchronizer {
    async fn sync_supply(
        &self,
        request: MintSupplySyncRequest,
    ) -> Result<(), MintSupplySynchronizerError> {
        let token_program_id = spl_token_2022_interface::id();
        let mint_authority = self.config.mint_authority().pubkey();
        let pool_account_owner = self.config.pool_account_owner().pubkey();
        let mint_address = Self::mint_address(&mint_authority, request.seed(), &token_program_id)
            .map_err(|error| {
            MintSupplySynchronizerError::Backend(Box::new(
                SolanaMintSupplySynchronizerError::MintAccountAddressDerivation(error),
            ))
        })?;
        let pool_token_account_address =
            Self::pool_token_account_address(&pool_account_owner, &mint_address, &token_program_id);
        let target_supply = u64::try_from(request.target_supply()).map_err(|_| {
            MintSupplySynchronizerError::Backend(Box::new(
                SolanaMintSupplySynchronizerError::TargetSupplyOverflow,
            ))
        })?;
        let current_supply = self
            .current_supply(&mint_address, &token_program_id)
            .await
            .map_err(|error| MintSupplySynchronizerError::Backend(Box::new(error)))?;

        if current_supply == target_supply {
            return Ok(());
        }

        let payer = self.config.payer().as_ref();
        let mint_authority_signer = self.config.mint_authority().as_ref();
        let pool_account_owner_signer = self.config.pool_account_owner().as_ref();
        let (instruction, signers): (Instruction, Vec<&dyn Signer>) =
            if current_supply < target_supply {
                let mint_amount = target_supply - current_supply;
                let instruction = token_instruction::mint_to_checked(
                    &token_program_id,
                    &mint_address,
                    &pool_token_account_address,
                    &mint_authority,
                    &[],
                    mint_amount,
                    request.decimals(),
                )
                .map_err(|error| {
                    MintSupplySynchronizerError::Backend(Box::new(
                        SolanaMintSupplySynchronizerError::MintToInstruction(error),
                    ))
                })?;
                let mut signers: Vec<&dyn Signer> = vec![payer];
                if payer.pubkey() != mint_authority_signer.pubkey() {
                    signers.push(mint_authority_signer);
                }
                (instruction, signers)
            } else {
                let burn_amount = current_supply - target_supply;
                let instruction = token_instruction::burn_checked(
                    &token_program_id,
                    &pool_token_account_address,
                    &mint_address,
                    &pool_account_owner,
                    &[],
                    burn_amount,
                    request.decimals(),
                )
                .map_err(|error| {
                    MintSupplySynchronizerError::Backend(Box::new(
                        SolanaMintSupplySynchronizerError::BurnInstruction(error),
                    ))
                })?;
                let mut signers: Vec<&dyn Signer> = vec![payer];
                if payer.pubkey() != pool_account_owner_signer.pubkey() {
                    signers.push(pool_account_owner_signer);
                }
                (instruction, signers)
            };

        self.send_transaction(vec![instruction], signers)
            .await
            .map_err(|error| MintSupplySynchronizerError::Backend(Box::new(error)))
    }
}
