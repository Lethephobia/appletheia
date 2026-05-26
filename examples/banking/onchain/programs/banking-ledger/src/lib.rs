pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;

declare_id!("DzYXFRU9PyJiEWLGaTQ8FA35urAtTkLH3G3QvQqMB2tZ");

#[program]
pub mod banking_ledger {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
