use std::sync::Arc;

use banking_ledger_application::{
    MintAccountAddress, MintAccountCreateReceipt, MintAccountCreateRequest, MintAccountCreator,
    MintAccountCreatorError,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    program_error::ProgramError,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use solana_system_interface::instruction as system_instruction;
use solana_transaction::Transaction;
use spl_token_2022_interface::extension::{
    BaseStateWithExtensions, ExtensionType, PodStateWithExtensions, metadata_pointer,
    metadata_pointer::MetadataPointer,
};
use spl_token_2022_interface::instruction as token_instruction;
use spl_token_2022_interface::pod::{PodCOption, PodMint};
use spl_token_metadata_interface::instruction as token_metadata_instruction;
use spl_token_metadata_interface::state::TokenMetadata;

use super::SolanaMintAccountCreatorError;

/// Solana implementation of `MintAccountCreator`.
pub struct SolanaMintAccountCreator {
    rpc_client: RpcClient,
    payer: Arc<Keypair>,
    mint_authority: Arc<Keypair>,
}

impl SolanaMintAccountCreator {
    pub fn new(rpc_client: RpcClient, payer: Arc<Keypair>, mint_authority: Arc<Keypair>) -> Self {
        Self {
            rpc_client,
            payer,
            mint_authority,
        }
    }

    pub fn new_with_rpc_url(
        rpc_url: String,
        payer: Arc<Keypair>,
        mint_authority: Arc<Keypair>,
    ) -> Self {
        Self::new(RpcClient::new(rpc_url), payer, mint_authority)
    }

    fn parse_token_program_id(
        request: &MintAccountCreateRequest,
    ) -> Result<Pubkey, SolanaMintAccountCreatorError> {
        request
            .token_program_id()
            .value()
            .parse()
            .map_err(SolanaMintAccountCreatorError::InvalidTokenProgramId)
    }

    fn parse_mint_authority(
        request: &MintAccountCreateRequest,
    ) -> Result<Pubkey, SolanaMintAccountCreatorError> {
        request
            .mint_authority()
            .value()
            .parse()
            .map_err(SolanaMintAccountCreatorError::InvalidMintAuthority)
    }

    fn parse_freeze_authority(
        request: &MintAccountCreateRequest,
    ) -> Result<Option<Pubkey>, SolanaMintAccountCreatorError> {
        request
            .freeze_authority()
            .map(|value| {
                value
                    .value()
                    .parse()
                    .map_err(SolanaMintAccountCreatorError::InvalidFreezeAuthority)
            })
            .transpose()
    }

    fn mint_address(
        mint_authority: &Pubkey,
        request: &MintAccountCreateRequest,
        token_program_id: &Pubkey,
    ) -> Result<Pubkey, SolanaMintAccountCreatorError> {
        Pubkey::create_with_seed(mint_authority, request.seed().value(), token_program_id)
            .map_err(SolanaMintAccountCreatorError::MintAccountAddressDerivation)
    }

    fn metadata_size(
        mint_address: &Pubkey,
        mint_authority: &Pubkey,
        request: &MintAccountCreateRequest,
    ) -> Result<usize, SolanaMintAccountCreatorError> {
        let metadata = TokenMetadata {
            update_authority: Some(*mint_authority).try_into().map_err(|_| {
                SolanaMintAccountCreatorError::MetadataSize(ProgramError::InvalidArgument)
            })?,
            mint: *mint_address,
            name: request.metadata().name().value().to_owned(),
            symbol: request.metadata().symbol().value().to_owned(),
            uri: request.metadata().uri().to_string(),
            additional_metadata: Vec::new(),
        };

        metadata
            .tlv_size_of()
            .map_err(SolanaMintAccountCreatorError::MetadataSize)
    }

    fn mint_account_size(
        mint_address: &Pubkey,
        mint_authority: &Pubkey,
        request: &MintAccountCreateRequest,
    ) -> Result<usize, SolanaMintAccountCreatorError> {
        let base_mint_size =
            ExtensionType::try_calculate_account_len::<PodMint>(&[ExtensionType::MetadataPointer])
                .map_err(SolanaMintAccountCreatorError::MintAccountSize)?;
        let metadata_size = Self::metadata_size(mint_address, mint_authority, request)?;

        base_mint_size
            .checked_add(metadata_size)
            .ok_or(SolanaMintAccountCreatorError::MintAccountSizeOverflow)
    }

    fn receipt(
        mint_address: &Pubkey,
        request: &MintAccountCreateRequest,
    ) -> Result<MintAccountCreateReceipt, SolanaMintAccountCreatorError> {
        Ok(MintAccountCreateReceipt::new(
            MintAccountAddress::try_from(mint_address.to_string())
                .map_err(SolanaMintAccountCreatorError::MintAccountAddress)?,
            request.token_program_id().clone(),
        ))
    }

    fn optional_pubkey(value: PodCOption<Pubkey>) -> Option<Pubkey> {
        if value.is_some() {
            Some(value.value)
        } else {
            None
        }
    }

    fn optional_pubkey_string(value: Option<Pubkey>) -> String {
        value
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_owned())
    }

    fn validate_existing_value(
        mint_address: &Pubkey,
        field: &'static str,
        actual: String,
        expected: String,
    ) -> Result<(), SolanaMintAccountCreatorError> {
        if actual == expected {
            return Ok(());
        }

        Err(SolanaMintAccountCreatorError::MintAccountUnexpectedValue {
            address: mint_address.to_string(),
            field,
            actual,
            expected,
        })
    }

    fn validate_existing_mint_account(
        mint_address: &Pubkey,
        mint_authority: &Pubkey,
        freeze_authority: Option<Pubkey>,
        request: &MintAccountCreateRequest,
        account_data: &[u8],
    ) -> Result<(), SolanaMintAccountCreatorError> {
        let state = PodStateWithExtensions::<PodMint>::unpack(account_data).map_err(|source| {
            SolanaMintAccountCreatorError::MintAccountInvalidData {
                address: mint_address.to_string(),
                source,
            }
        })?;
        Self::validate_existing_value(
            mint_address,
            "decimals",
            state.base.decimals.to_string(),
            request.decimals().to_string(),
        )?;
        Self::validate_existing_value(
            mint_address,
            "mint_authority",
            Self::optional_pubkey_string(Self::optional_pubkey(state.base.mint_authority)),
            Self::optional_pubkey_string(Some(*mint_authority)),
        )?;
        Self::validate_existing_value(
            mint_address,
            "freeze_authority",
            Self::optional_pubkey_string(Self::optional_pubkey(state.base.freeze_authority)),
            Self::optional_pubkey_string(freeze_authority),
        )?;

        let metadata_pointer = state.get_extension::<MetadataPointer>().map_err(|source| {
            SolanaMintAccountCreatorError::MintAccountInvalidData {
                address: mint_address.to_string(),
                source,
            }
        })?;
        Self::validate_existing_value(
            mint_address,
            "metadata_pointer_authority",
            Self::optional_pubkey_string(metadata_pointer.authority.copied()),
            Self::optional_pubkey_string(Some(*mint_authority)),
        )?;
        Self::validate_existing_value(
            mint_address,
            "metadata_pointer_address",
            Self::optional_pubkey_string(metadata_pointer.metadata_address.copied()),
            Self::optional_pubkey_string(Some(*mint_address)),
        )?;

        let metadata = state
            .get_variable_len_extension::<TokenMetadata>()
            .map_err(
                |source| SolanaMintAccountCreatorError::MintAccountInvalidData {
                    address: mint_address.to_string(),
                    source,
                },
            )?;
        Self::validate_existing_value(
            mint_address,
            "metadata_update_authority",
            Self::optional_pubkey_string(metadata.update_authority.copied()),
            Self::optional_pubkey_string(Some(*mint_authority)),
        )?;
        Self::validate_existing_value(
            mint_address,
            "metadata_mint",
            metadata.mint.to_string(),
            mint_address.to_string(),
        )?;
        Self::validate_existing_value(
            mint_address,
            "metadata_name",
            metadata.name,
            request.metadata().name().value().to_owned(),
        )?;
        Self::validate_existing_value(
            mint_address,
            "metadata_symbol",
            metadata.symbol,
            request.metadata().symbol().value().to_owned(),
        )?;
        Self::validate_existing_value(
            mint_address,
            "metadata_uri",
            metadata.uri,
            request.metadata().uri().to_string(),
        )?;
        if !metadata.additional_metadata.is_empty() {
            return Err(SolanaMintAccountCreatorError::MintAccountUnexpectedValue {
                address: mint_address.to_string(),
                field: "metadata_additional_metadata",
                actual: format!("{} entries", metadata.additional_metadata.len()),
                expected: "0 entries".to_owned(),
            });
        }

        Ok(())
    }

    async fn find_existing_receipt(
        &self,
        mint_address: &Pubkey,
        token_program_id: &Pubkey,
        mint_authority: &Pubkey,
        freeze_authority: Option<Pubkey>,
        request: &MintAccountCreateRequest,
    ) -> Result<Option<MintAccountCreateReceipt>, SolanaMintAccountCreatorError> {
        let Some(account) = self
            .rpc_client
            .get_account_with_commitment(mint_address, self.rpc_client.commitment())
            .await?
            .value
        else {
            return Ok(None);
        };

        if account.owner != *token_program_id {
            return Err(SolanaMintAccountCreatorError::MintAccountUnexpectedOwner {
                address: mint_address.to_string(),
                owner: account.owner.to_string(),
                expected_owner: token_program_id.to_string(),
            });
        }

        Self::validate_existing_mint_account(
            mint_address,
            mint_authority,
            freeze_authority,
            request,
            &account.data,
        )?;

        Self::receipt(mint_address, request).map(Some)
    }

    async fn create_or_get_inner(
        &self,
        request: MintAccountCreateRequest,
    ) -> Result<MintAccountCreateReceipt, SolanaMintAccountCreatorError> {
        let token_program_id = Self::parse_token_program_id(&request)?;
        let request_mint_authority = Self::parse_mint_authority(&request)?;
        let freeze_authority = Self::parse_freeze_authority(&request)?;
        let mint_authority = self.mint_authority.pubkey();

        if request_mint_authority != mint_authority {
            return Err(SolanaMintAccountCreatorError::MintAuthorityMismatch {
                signer: mint_authority.to_string(),
                request: request_mint_authority.to_string(),
            });
        }

        let mint_address = Self::mint_address(&mint_authority, &request, &token_program_id)?;

        if let Some(receipt) = self
            .find_existing_receipt(
                &mint_address,
                &token_program_id,
                &mint_authority,
                freeze_authority,
                &request,
            )
            .await?
        {
            return Ok(receipt);
        }

        let space = Self::mint_account_size(&mint_address, &mint_authority, &request)?;
        let lamports = self
            .rpc_client
            .get_minimum_balance_for_rent_exemption(space)
            .await?;
        let payer = self.payer.pubkey();
        let create_mint_account = system_instruction::create_account_with_seed(
            &payer,
            &mint_address,
            &mint_authority,
            request.seed().value(),
            lamports,
            space as u64,
            &token_program_id,
        );
        let initialize_metadata_pointer = metadata_pointer::instruction::initialize(
            &token_program_id,
            &mint_address,
            Some(mint_authority),
            Some(mint_address),
        )
        .map_err(SolanaMintAccountCreatorError::MetadataPointerInstruction)?;
        let initialize_mint = token_instruction::initialize_mint2(
            &token_program_id,
            &mint_address,
            &mint_authority,
            freeze_authority.as_ref(),
            request.decimals(),
        )
        .map_err(SolanaMintAccountCreatorError::InitializeMintInstruction)?;
        let initialize_metadata = token_metadata_instruction::initialize(
            &token_program_id,
            &mint_address,
            &mint_authority,
            &mint_address,
            &mint_authority,
            request.metadata().name().value().to_owned(),
            request.metadata().symbol().value().to_owned(),
            request.metadata().uri().to_string(),
        );
        let instructions = vec![
            create_mint_account,
            initialize_metadata_pointer,
            initialize_mint,
            initialize_metadata,
        ];
        let blockhash = self.rpc_client.get_latest_blockhash().await?;
        let transaction = {
            let payer = self.payer.as_ref();
            let mint_authority = self.mint_authority.as_ref();
            let signers: Vec<&dyn Signer> = if payer.pubkey() == mint_authority.pubkey() {
                vec![payer]
            } else {
                vec![payer, mint_authority]
            };
            let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
            transaction.try_sign(&signers, blockhash)?;
            transaction
        };

        if let Err(error) = self
            .rpc_client
            .send_and_confirm_transaction(&transaction)
            .await
        {
            if let Some(receipt) = self
                .find_existing_receipt(
                    &mint_address,
                    &token_program_id,
                    &mint_authority,
                    freeze_authority,
                    &request,
                )
                .await?
            {
                return Ok(receipt);
            }

            return Err(error.into());
        }

        Self::receipt(&mint_address, &request)
    }
}

impl MintAccountCreator for SolanaMintAccountCreator {
    async fn create_or_get(
        &self,
        request: MintAccountCreateRequest,
    ) -> Result<MintAccountCreateReceipt, MintAccountCreatorError> {
        self.create_or_get_inner(request)
            .await
            .map_err(|error| MintAccountCreatorError::Backend(Box::new(error)))
    }
}

#[cfg(test)]
mod tests {
    use banking_ledger_application::{
        MintAccountMetadata, MintAccountSeed, MintMetadataName, MintMetadataSymbol,
        MintMetadataUri, OnchainAccountAddress, TokenProgramId,
    };
    use solana_sdk::pubkey::Pubkey;
    use spl_token_2022_interface::extension::{
        BaseStateWithExtensionsMut, ExtensionType, PodStateWithExtensionsMut,
        metadata_pointer::MetadataPointer,
    };
    use spl_token_2022_interface::pod::{PodCOption, PodMint};
    use spl_token_metadata_interface::state::TokenMetadata;

    use super::{SolanaMintAccountCreator, SolanaMintAccountCreatorError};

    const TOKEN_PROGRAM_ID: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";
    const METADATA_URI: &str = "https://metadata.example.com/currencies/test/mint/metadata.json";

    fn request(
        seed: MintAccountSeed,
        mint_authority: &Pubkey,
        freeze_authority: Option<&Pubkey>,
    ) -> banking_ledger_application::MintAccountCreateRequest {
        banking_ledger_application::MintAccountCreateRequest::new(
            seed,
            6,
            TokenProgramId::try_from(TOKEN_PROGRAM_ID).expect("token program ID should be valid"),
            OnchainAccountAddress::try_from(mint_authority.to_string())
                .expect("mint authority should be valid"),
            freeze_authority.map(|value| {
                OnchainAccountAddress::try_from(value.to_string())
                    .expect("freeze authority should be valid")
            }),
            MintAccountMetadata::new(
                MintMetadataName::try_from("USD Coin").expect("metadata name should be valid"),
                MintMetadataSymbol::try_from("USDC").expect("metadata symbol should be valid"),
                MintMetadataUri::try_from(METADATA_URI).expect("metadata URI should be valid"),
            ),
        )
    }

    fn mint_address(
        mint_authority: &Pubkey,
        request: &banking_ledger_application::MintAccountCreateRequest,
    ) -> Pubkey {
        let token_program_id = request
            .token_program_id()
            .value()
            .parse()
            .expect("token program ID should parse");

        Pubkey::create_with_seed(mint_authority, request.seed().value(), &token_program_id)
            .expect("mint address should derive")
    }

    fn matching_mint_account_data(
        mint_address: &Pubkey,
        mint_authority: &Pubkey,
        freeze_authority: Option<Pubkey>,
        request: &banking_ledger_application::MintAccountCreateRequest,
    ) -> Vec<u8> {
        let space =
            SolanaMintAccountCreator::mint_account_size(mint_address, mint_authority, request)
                .expect("mint account size should be calculated");
        let mut buffer = vec![0; space];
        let mut state = PodStateWithExtensionsMut::<PodMint>::unpack_uninitialized(&mut buffer)
            .expect("mint state should unpack");
        *state.base = PodMint {
            mint_authority: PodCOption::some(*mint_authority),
            supply: 0.into(),
            decimals: request.decimals(),
            is_initialized: true.into(),
            freeze_authority: freeze_authority
                .map(PodCOption::some)
                .unwrap_or_else(PodCOption::none),
        };
        state
            .init_account_type()
            .expect("account type should be initialized");

        let metadata_pointer = state
            .init_extension::<MetadataPointer>(false)
            .expect("metadata pointer should initialize");
        metadata_pointer.authority = Some(*mint_authority)
            .try_into()
            .expect("metadata pointer authority should be valid");
        metadata_pointer.metadata_address = Some(*mint_address)
            .try_into()
            .expect("metadata pointer address should be valid");

        let metadata = TokenMetadata {
            update_authority: Some(*mint_authority)
                .try_into()
                .expect("metadata authority should be valid"),
            mint: *mint_address,
            name: request.metadata().name().value().to_owned(),
            symbol: request.metadata().symbol().value().to_owned(),
            uri: request.metadata().uri().to_string(),
            additional_metadata: Vec::new(),
        };
        state
            .init_variable_len_extension(&metadata, false)
            .expect("metadata should initialize");

        buffer
    }

    #[test]
    fn validate_existing_mint_account_accepts_matching_account_data() {
        let seed = MintAccountSeed::try_from("00000000000000000000000000000000")
            .expect("seed should be valid");
        let mint_authority = Pubkey::new_unique();
        let freeze_authority = Some(Pubkey::new_unique());
        let request = request(seed, &mint_authority, freeze_authority.as_ref());
        let mint_address = mint_address(&mint_authority, &request);
        let account_data =
            matching_mint_account_data(&mint_address, &mint_authority, freeze_authority, &request);

        SolanaMintAccountCreator::validate_existing_mint_account(
            &mint_address,
            &mint_authority,
            freeze_authority,
            &request,
            &account_data,
        )
        .expect("matching account data should validate");
    }

    #[test]
    fn validate_existing_mint_account_rejects_metadata_uri_mismatch() {
        let seed = MintAccountSeed::try_from("00000000000000000000000000000000")
            .expect("seed should be valid");
        let mint_authority = Pubkey::new_unique();
        let request = request(seed, &mint_authority, None);
        let mint_address = mint_address(&mint_authority, &request);
        let account_data =
            matching_mint_account_data(&mint_address, &mint_authority, None, &request);
        let mut mismatched_request = request.clone();
        mismatched_request = banking_ledger_application::MintAccountCreateRequest::new(
            mismatched_request.seed().clone(),
            mismatched_request.decimals(),
            mismatched_request.token_program_id().clone(),
            mismatched_request.mint_authority().clone(),
            mismatched_request.freeze_authority().cloned(),
            MintAccountMetadata::new(
                mismatched_request.metadata().name().clone(),
                mismatched_request.metadata().symbol().clone(),
                MintMetadataUri::try_from(
                    "https://metadata.example.com/currencies/test/mint/changed.json",
                )
                .expect("metadata URI should be valid"),
            ),
        );

        let error = SolanaMintAccountCreator::validate_existing_mint_account(
            &mint_address,
            &mint_authority,
            None,
            &mismatched_request,
            &account_data,
        )
        .expect_err("metadata URI mismatch should be rejected");

        assert!(matches!(
            error,
            SolanaMintAccountCreatorError::MintAccountUnexpectedValue {
                field: "metadata_uri",
                ..
            }
        ));
    }

    #[test]
    fn validate_existing_mint_account_rejects_missing_metadata_pointer() {
        let seed = MintAccountSeed::try_from("00000000000000000000000000000000")
            .expect("seed should be valid");
        let mint_authority = Pubkey::new_unique();
        let request = request(seed, &mint_authority, None);
        let mint_address = mint_address(&mint_authority, &request);
        let space = ExtensionType::try_calculate_account_len::<PodMint>(&[])
            .expect("mint account size should be calculated");
        let mut account_data = vec![0; space];
        let mut state =
            PodStateWithExtensionsMut::<PodMint>::unpack_uninitialized(&mut account_data)
                .expect("mint state should unpack");
        *state.base = PodMint {
            mint_authority: PodCOption::some(mint_authority),
            supply: 0.into(),
            decimals: request.decimals(),
            is_initialized: true.into(),
            freeze_authority: PodCOption::none(),
        };
        state
            .init_account_type()
            .expect("account type should be initialized");

        let error = SolanaMintAccountCreator::validate_existing_mint_account(
            &mint_address,
            &mint_authority,
            None,
            &request,
            &account_data,
        )
        .expect_err("missing metadata pointer should be rejected");

        assert!(matches!(
            error,
            SolanaMintAccountCreatorError::MintAccountInvalidData { .. }
        ));
    }
}
