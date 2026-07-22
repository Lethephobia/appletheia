use anchor_lang::prelude::*;

use crate::instruction_handler::PoolTokenAccountEnsureInstructionAccounts;

pub(crate) struct PoolTokenAccountEnsureInstructionHandler;

impl PoolTokenAccountEnsureInstructionHandler {
    pub(crate) fn handle(
        _ctx: Context<PoolTokenAccountEnsureInstructionAccounts>,
        _mint_id: [u8; 16],
    ) -> Result<()> {
        Ok(())
    }
}
