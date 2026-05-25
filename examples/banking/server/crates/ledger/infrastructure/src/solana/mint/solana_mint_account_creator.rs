use banking_ledger_application::{
    MintAccountAddress, MintAccountCreateReceipt, MintAccountCreateRequest, MintAccountCreator,
    MintAccountCreatorError, MintAccountMetadata, MintAccountSeed, OnchainAccountAddress,
    TokenProgramId,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction, program_error::ProgramError, pubkey::Pubkey, pubkey::PubkeyError,
    signature::Signer,
};
use solana_system_interface::instruction as system_instruction;
use solana_transaction::Transaction;
use spl_token_2022_interface::extension::{
    ExtensionType, PodStateWithExtensions, metadata_pointer,
};
use spl_token_2022_interface::instruction as token_instruction;
use spl_token_2022_interface::pod::{PodAccount, PodMint};
use spl_token_2022_interface::state::PackedSizeOf;
use spl_token_metadata_interface::instruction as token_metadata_instruction;
use spl_token_metadata_interface::state::TokenMetadata;

use super::{SolanaMintAccountCreatorConfig, SolanaMintAccountCreatorError};

/// Solana implementation of `MintAccountCreator`.
pub struct SolanaMintAccountCreator {
    rpc_client: RpcClient,
    config: SolanaMintAccountCreatorConfig,
}

impl SolanaMintAccountCreator {
    pub fn new(rpc_client: RpcClient, config: SolanaMintAccountCreatorConfig) -> Self {
        Self { rpc_client, config }
    }

    fn mint_address(
        mint_authority: &Pubkey,
        seed: &MintAccountSeed,
        token_program_id: &Pubkey,
    ) -> Result<Pubkey, PubkeyError> {
        Pubkey::create_with_seed(mint_authority, seed.value(), token_program_id)
    }

    fn pool_address(
        pool_account_owner: &Pubkey,
        seed: &MintAccountSeed,
        token_program_id: &Pubkey,
    ) -> Result<Pubkey, PubkeyError> {
        Pubkey::create_with_seed(pool_account_owner, seed.value(), token_program_id)
    }

    fn metadata_size(
        mint_address: &Pubkey,
        mint_authority: &Pubkey,
        metadata: &MintAccountMetadata,
    ) -> Result<usize, SolanaMintAccountCreatorError> {
        let metadata = TokenMetadata {
            update_authority: Some(*mint_authority).try_into().map_err(|_| {
                SolanaMintAccountCreatorError::MetadataSize(ProgramError::InvalidArgument)
            })?,
            mint: *mint_address,
            name: metadata.name().value().to_owned(),
            symbol: metadata.symbol().value().to_owned(),
            uri: metadata.uri().to_string(),
            additional_metadata: Vec::new(),
        };

        metadata
            .tlv_size_of()
            .map_err(SolanaMintAccountCreatorError::MetadataSize)
    }

    fn mint_account_size(
        mint_address: &Pubkey,
        mint_authority: &Pubkey,
        metadata: &MintAccountMetadata,
    ) -> Result<usize, SolanaMintAccountCreatorError> {
        let base_mint_size =
            ExtensionType::try_calculate_account_len::<PodMint>(&[ExtensionType::MetadataPointer])
                .map_err(SolanaMintAccountCreatorError::MintAccountSize)?;
        let metadata_size = Self::metadata_size(mint_address, mint_authority, metadata)?;

        base_mint_size
            .checked_add(metadata_size)
            .ok_or(SolanaMintAccountCreatorError::MintAccountSizeOverflow)
    }

    async fn mint_account_exists(
        &self,
        mint_address: &Pubkey,
        token_program_id: &Pubkey,
    ) -> Result<bool, MintAccountCreatorError> {
        let Some(account) = self
            .rpc_client
            .get_account_with_commitment(mint_address, self.rpc_client.commitment())
            .await
            .map_err(|error| MintAccountCreatorError::Backend(Box::new(error)))?
            .value
        else {
            return Ok(false);
        };

        if account.owner != *token_program_id {
            return Err(MintAccountCreatorError::Backend(Box::new(
                SolanaMintAccountCreatorError::MintAccountUnexpectedOwner {
                    address: mint_address.to_string(),
                    owner: account.owner.to_string(),
                    expected_owner: token_program_id.to_string(),
                },
            )));
        }

        PodStateWithExtensions::<PodMint>::unpack(&account.data)
            .map_err(|error| MintAccountCreatorError::Backend(Box::new(error)))?;

        Ok(true)
    }

    async fn pool_account_exists(
        &self,
        pool_address: &Pubkey,
        token_program_id: &Pubkey,
    ) -> Result<bool, MintAccountCreatorError> {
        let Some(account) = self
            .rpc_client
            .get_account_with_commitment(pool_address, self.rpc_client.commitment())
            .await
            .map_err(|error| MintAccountCreatorError::Backend(Box::new(error)))?
            .value
        else {
            return Ok(false);
        };

        if account.owner != *token_program_id {
            return Err(MintAccountCreatorError::Backend(Box::new(
                SolanaMintAccountCreatorError::PoolAccountUnexpectedOwner {
                    address: pool_address.to_string(),
                    owner: account.owner.to_string(),
                    expected_owner: token_program_id.to_string(),
                },
            )));
        }

        PodStateWithExtensions::<PodAccount>::unpack(&account.data).map_err(|source| {
            MintAccountCreatorError::Backend(Box::new(
                SolanaMintAccountCreatorError::PoolAccountInvalidData {
                    address: pool_address.to_string(),
                    source,
                },
            ))
        })?;

        Ok(true)
    }

    async fn send_transaction(
        &self,
        instructions: Vec<Instruction>,
    ) -> Result<(), SolanaMintAccountCreatorError> {
        let blockhash = self.rpc_client.get_latest_blockhash().await?;
        let transaction = {
            let payer = self.config.payer().as_ref();
            let mint_authority = self.config.mint_authority().as_ref();
            let pool_account_owner = self.config.pool_account_owner().as_ref();
            let mut signers: Vec<&dyn Signer> = vec![payer];
            if payer.pubkey() != mint_authority.pubkey() {
                signers.push(mint_authority);
            }
            if pool_account_owner.pubkey() != payer.pubkey()
                && pool_account_owner.pubkey() != mint_authority.pubkey()
            {
                signers.push(pool_account_owner);
            }
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

impl MintAccountCreator for SolanaMintAccountCreator {
    // This adapter still reconciles against existing on-chain accounts because local persistence
    // can fail after the transaction succeeds and the workflow may retry later.
    async fn create_or_get(
        &self,
        request: MintAccountCreateRequest,
    ) -> Result<MintAccountCreateReceipt, MintAccountCreatorError> {
        let token_program_id = spl_token_2022_interface::id();
        let mint_authority = self.config.mint_authority().pubkey();
        let pool_account_owner = self.config.pool_account_owner().pubkey();
        let freeze_authority = self.config.freeze_authority();
        let mint_address = Self::mint_address(&mint_authority, request.seed(), &token_program_id)
            .map_err(|error| MintAccountCreatorError::Backend(Box::new(error)))?;
        let pool_address =
            Self::pool_address(&pool_account_owner, request.seed(), &token_program_id)
                .map_err(|error| MintAccountCreatorError::Backend(Box::new(error)))?;

        let mint_exists = self
            .mint_account_exists(&mint_address, &token_program_id)
            .await?;
        let pool_exists = self
            .pool_account_exists(&pool_address, &token_program_id)
            .await?;

        if mint_exists && pool_exists {
            return Ok(MintAccountCreateReceipt::new(
                MintAccountAddress::try_from(mint_address.to_string())
                    .map_err(|error| MintAccountCreatorError::Backend(Box::new(error)))?,
                OnchainAccountAddress::try_from(pool_address.to_string())
                    .map_err(|error| MintAccountCreatorError::Backend(Box::new(error)))?,
                TokenProgramId::try_from(token_program_id.to_string())
                    .map_err(|error| MintAccountCreatorError::Backend(Box::new(error)))?,
            ));
        }

        let payer = self.config.payer().pubkey();
        let mut instructions = Vec::new();

        if !mint_exists {
            let mint_space =
                Self::mint_account_size(&mint_address, &mint_authority, request.metadata())
                    .map_err(|error| MintAccountCreatorError::Backend(Box::new(error)))?;
            let lamports = self
                .rpc_client
                .get_minimum_balance_for_rent_exemption(mint_space)
                .await
                .map_err(|error| MintAccountCreatorError::Backend(Box::new(error)))?;
            instructions.push(system_instruction::create_account_with_seed(
                &payer,
                &mint_address,
                &mint_authority,
                request.seed().value(),
                lamports,
                mint_space as u64,
                &token_program_id,
            ));
            instructions.push(
                metadata_pointer::instruction::initialize(
                    &token_program_id,
                    &mint_address,
                    Some(mint_authority),
                    Some(mint_address),
                )
                .map_err(|error| MintAccountCreatorError::Backend(Box::new(error)))?,
            );
            instructions.push(
                token_instruction::initialize_mint2(
                    &token_program_id,
                    &mint_address,
                    &mint_authority,
                    freeze_authority.as_ref(),
                    request.decimals().value(),
                )
                .map_err(|error| MintAccountCreatorError::Backend(Box::new(error)))?,
            );
            instructions.push(token_metadata_instruction::initialize(
                &token_program_id,
                &mint_address,
                &mint_authority,
                &mint_address,
                &mint_authority,
                request.metadata().name().value().to_owned(),
                request.metadata().symbol().value().to_owned(),
                request.metadata().uri().to_string(),
            ));
        }

        if !pool_exists {
            let pool_space = PodAccount::SIZE_OF;
            let lamports = self
                .rpc_client
                .get_minimum_balance_for_rent_exemption(pool_space)
                .await
                .map_err(|error| MintAccountCreatorError::Backend(Box::new(error)))?;
            instructions.push(system_instruction::create_account_with_seed(
                &payer,
                &pool_address,
                &pool_account_owner,
                request.seed().value(),
                lamports,
                pool_space as u64,
                &token_program_id,
            ));
            instructions.push(
                token_instruction::initialize_account3(
                    &token_program_id,
                    &pool_address,
                    &mint_address,
                    &pool_account_owner,
                )
                .map_err(|error| {
                    MintAccountCreatorError::Backend(Box::new(
                        SolanaMintAccountCreatorError::InitializePoolAccountInstruction(error),
                    ))
                })?,
            );
        }

        if let Err(error) = self.send_transaction(instructions).await {
            if let (Ok(true), Ok(true)) = (
                self.mint_account_exists(&mint_address, &token_program_id)
                    .await,
                self.pool_account_exists(&pool_address, &token_program_id)
                    .await,
            ) {
                return Ok(MintAccountCreateReceipt::new(
                    MintAccountAddress::try_from(mint_address.to_string())
                        .map_err(|error| MintAccountCreatorError::Backend(Box::new(error)))?,
                    OnchainAccountAddress::try_from(pool_address.to_string())
                        .map_err(|error| MintAccountCreatorError::Backend(Box::new(error)))?,
                    TokenProgramId::try_from(token_program_id.to_string())
                        .map_err(|error| MintAccountCreatorError::Backend(Box::new(error)))?,
                ));
            }

            return Err(MintAccountCreatorError::Backend(Box::new(error)));
        }

        Ok(MintAccountCreateReceipt::new(
            MintAccountAddress::try_from(mint_address.to_string())
                .map_err(|error| MintAccountCreatorError::Backend(Box::new(error)))?,
            OnchainAccountAddress::try_from(pool_address.to_string())
                .map_err(|error| MintAccountCreatorError::Backend(Box::new(error)))?,
            TokenProgramId::try_from(token_program_id.to_string())
                .map_err(|error| MintAccountCreatorError::Backend(Box::new(error)))?,
        ))
    }
}
