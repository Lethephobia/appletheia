use anchor_lang::context::Context;
use anchor_lang::{Bumps, Result};

pub trait InstructionHandler {
    type Accounts<'info>: Bumps;
    type Args;

    fn handle<'context, 'info>(
        ctx: Context<'context, Self::Accounts<'info>>,
        args: Self::Args,
    ) -> Result<()>;
}
