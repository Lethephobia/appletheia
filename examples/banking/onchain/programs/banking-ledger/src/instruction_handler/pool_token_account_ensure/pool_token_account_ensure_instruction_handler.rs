use anchor_lang::prelude::*;
use banking_anchor::instruction::InstructionHandler;

use crate::instruction_handler::{
    PoolTokenAccountEnsureInstructionAccounts, PoolTokenAccountEnsureInstructionArgs,
};

pub(crate) struct PoolTokenAccountEnsureInstructionHandler;

impl InstructionHandler for PoolTokenAccountEnsureInstructionHandler {
    type Accounts<'info> = PoolTokenAccountEnsureInstructionAccounts<'info>;
    type Args = PoolTokenAccountEnsureInstructionArgs;

    fn handle<'context, 'info>(
        _ctx: Context<'context, Self::Accounts<'info>>,
        _args: Self::Args,
    ) -> Result<()> {
        Ok(())
    }
}
