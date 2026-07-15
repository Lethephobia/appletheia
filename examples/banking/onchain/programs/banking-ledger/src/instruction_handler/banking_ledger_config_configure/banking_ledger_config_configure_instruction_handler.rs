use anchor_lang::prelude::*;
use banking_anchor::instruction::InstructionHandler;

use crate::account::BankingLedgerConfigInitialization;
use crate::instruction_handler::{
    BankingLedgerConfigConfigureInstructionAccounts, BankingLedgerConfigConfigureInstructionArgs,
};

pub(crate) struct BankingLedgerConfigConfigureInstructionHandler;

impl InstructionHandler for BankingLedgerConfigConfigureInstructionHandler {
    type Accounts<'info> = BankingLedgerConfigConfigureInstructionAccounts<'info>;
    type Args = BankingLedgerConfigConfigureInstructionArgs;

    fn handle<'context, 'info>(
        ctx: Context<'context, Self::Accounts<'info>>,
        _args: Self::Args,
    ) -> Result<()> {
        let operator = ctx.accounts.operator.key();

        if ctx.accounts.banking_ledger_config.is_initialized() {
            ctx.accounts.banking_ledger_config.change_operator(operator);

            return Ok(());
        }

        ctx.accounts
            .banking_ledger_config
            .initialize(BankingLedgerConfigInitialization {
                operator,
                bump: ctx.bumps.banking_ledger_config,
            });

        Ok(())
    }
}
