use anchor_lang::prelude::*;

use crate::account::BankingLedgerConfigInitialization;
use crate::instruction_handler::BankingLedgerConfigConfigureInstructionAccounts;

pub(crate) struct BankingLedgerConfigConfigureInstructionHandler;

impl BankingLedgerConfigConfigureInstructionHandler {
    pub(crate) fn handle(
        ctx: Context<BankingLedgerConfigConfigureInstructionAccounts>,
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
