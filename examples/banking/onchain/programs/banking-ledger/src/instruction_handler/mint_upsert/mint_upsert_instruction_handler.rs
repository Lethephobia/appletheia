use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use spl_token_metadata_interface::instruction as token_metadata_instruction;
use spl_token_metadata_interface::state::{Field, TokenMetadata};
use spl_type_length_value::state::{TlvState, TlvStateBorrowed};

use crate::account::{MintMetadata, MintStateInitialization, ProgramAuthority};
use crate::instruction_handler::{MintUpsertInstructionAccounts, MintUpsertInstructionError};

pub(crate) struct MintUpsertInstructionHandler;

impl MintUpsertInstructionHandler {
    fn token_metadata(mint_metadata: &AccountInfo<'_>) -> Result<TokenMetadata> {
        let data = mint_metadata.try_borrow_data()?;
        let state = TlvStateBorrowed::unpack(&data)?;

        Ok(state.get_first_variable_len_value::<TokenMetadata>()?)
    }

    fn update_metadata_field(
        token_program_id: &Pubkey,
        mint_metadata: &Pubkey,
        program_authority: &Pubkey,
        field: Field,
        value: String,
        accounts: &[AccountInfo<'_>],
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        invoke_signed(
            &token_metadata_instruction::update_field(
                token_program_id,
                mint_metadata,
                program_authority,
                field,
                value,
            ),
            accounts,
            signer_seeds,
        )?;

        Ok(())
    }

    pub(crate) fn handle(
        ctx: Context<MintUpsertInstructionAccounts>,
        _mint_id: [u8; 16],
        _decimals: u8,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        require!(
            name.len() <= MintMetadata::MAX_NAME_BYTES,
            MintUpsertInstructionError::MetadataNameTooLong
        );
        require!(
            symbol.len() <= MintMetadata::MAX_SYMBOL_BYTES,
            MintUpsertInstructionError::MetadataSymbolTooLong
        );
        require!(
            uri.len() <= MintMetadata::MAX_URI_BYTES,
            MintUpsertInstructionError::MetadataUriTooLong
        );

        let token_program_id = ctx.accounts.token_program.key();

        let program_authority = ctx.accounts.program_authority.key();
        let mint = ctx.accounts.mint.key();
        let mint_metadata = ctx.accounts.mint_metadata.key();

        if ctx.accounts.mint_state.is_initialized() {
            let current_metadata =
                Self::token_metadata(&ctx.accounts.mint_metadata.to_account_info())?;

            let authority_seeds = &[ProgramAuthority::SEED, &[ctx.bumps.program_authority]];
            let metadata_accounts = &[
                ctx.accounts.mint_metadata.to_account_info(),
                ctx.accounts.program_authority.to_account_info(),
            ];

            if current_metadata.name != name {
                Self::update_metadata_field(
                    &token_program_id,
                    &mint_metadata,
                    &program_authority,
                    Field::Name,
                    name,
                    metadata_accounts,
                    &[authority_seeds],
                )?;
            }
            if current_metadata.symbol != symbol {
                Self::update_metadata_field(
                    &token_program_id,
                    &mint_metadata,
                    &program_authority,
                    Field::Symbol,
                    symbol,
                    metadata_accounts,
                    &[authority_seeds],
                )?;
            }
            if current_metadata.uri != uri {
                Self::update_metadata_field(
                    &token_program_id,
                    &mint_metadata,
                    &program_authority,
                    Field::Uri,
                    uri,
                    metadata_accounts,
                    &[authority_seeds],
                )?;
            }

            return Ok(());
        }

        ctx.accounts.mint_state.initialize(MintStateInitialization {
            bump: ctx.bumps.mint_state,
            mint_bump: ctx.bumps.mint,
            mint_metadata_bump: ctx.bumps.mint_metadata,
            program_authority_bump: ctx.bumps.program_authority,
        });

        let authority_seeds = &[ProgramAuthority::SEED, &[ctx.bumps.program_authority]];

        invoke_signed(
            &token_metadata_instruction::initialize(
                &token_program_id,
                &mint_metadata,
                &program_authority,
                &mint,
                &program_authority,
                name,
                symbol,
                uri,
            ),
            &[
                ctx.accounts.mint_metadata.to_account_info(),
                ctx.accounts.program_authority.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.program_authority.to_account_info(),
            ],
            &[authority_seeds],
        )?;

        Ok(())
    }
}
