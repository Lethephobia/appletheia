use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use banking_anchor::instruction::InstructionHandler;
use spl_token_metadata_interface::instruction as token_metadata_instruction;
use spl_token_metadata_interface::state::Field;

use crate::account::{MintMetadata, ProgramAuthority};
use crate::instruction_handler::{
    MintMetadataUpdateInstructionAccounts, MintMetadataUpdateInstructionArgs,
    MintMetadataUpdateInstructionError,
};

pub(crate) struct MintMetadataUpdateInstructionHandler;

impl MintMetadataUpdateInstructionHandler {
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
}

impl InstructionHandler for MintMetadataUpdateInstructionHandler {
    type Accounts<'info> = MintMetadataUpdateInstructionAccounts<'info>;
    type Args = MintMetadataUpdateInstructionArgs;

    fn handle<'context, 'info>(
        ctx: Context<'context, Self::Accounts<'info>>,
        args: Self::Args,
    ) -> Result<()> {
        let MintMetadataUpdateInstructionArgs {
            mint_id: _,
            name,
            symbol,
            uri,
        } = args;

        require!(
            name.len() <= MintMetadata::MAX_NAME_BYTES,
            MintMetadataUpdateInstructionError::MetadataNameTooLong
        );
        require!(
            symbol.len() <= MintMetadata::MAX_SYMBOL_BYTES,
            MintMetadataUpdateInstructionError::MetadataSymbolTooLong
        );
        require!(
            uri.len() <= MintMetadata::MAX_URI_BYTES,
            MintMetadataUpdateInstructionError::MetadataUriTooLong
        );

        let token_program_id = ctx.accounts.token_program.key();
        let program_authority = ctx.accounts.program_authority.key();
        let mint_metadata = ctx.accounts.mint_metadata.key();
        let authority_seeds = &[
            ProgramAuthority::SEED,
            &[ctx.accounts.mint_state.program_authority_bump],
        ];
        let metadata_accounts = &[
            ctx.accounts.mint_metadata.to_account_info(),
            ctx.accounts.program_authority.to_account_info(),
        ];

        Self::update_metadata_field(
            &token_program_id,
            &mint_metadata,
            &program_authority,
            Field::Name,
            name,
            metadata_accounts,
            &[authority_seeds],
        )?;
        Self::update_metadata_field(
            &token_program_id,
            &mint_metadata,
            &program_authority,
            Field::Symbol,
            symbol,
            metadata_accounts,
            &[authority_seeds],
        )?;
        Self::update_metadata_field(
            &token_program_id,
            &mint_metadata,
            &program_authority,
            Field::Uri,
            uri,
            metadata_accounts,
            &[authority_seeds],
        )?;

        Ok(())
    }
}
