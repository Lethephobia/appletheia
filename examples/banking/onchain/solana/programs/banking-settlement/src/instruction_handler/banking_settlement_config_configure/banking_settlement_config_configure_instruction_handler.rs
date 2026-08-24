use anchor_lang::prelude::*;

use crate::account::BankingSettlementConfigInitialization;
use crate::instruction_handler::BankingSettlementConfigConfigureInstructionAccounts;

pub(crate) struct BankingSettlementConfigConfigureInstructionHandler;

impl BankingSettlementConfigConfigureInstructionHandler {
    pub(crate) fn handle(
        ctx: Context<BankingSettlementConfigConfigureInstructionAccounts>,
    ) -> Result<()> {
        let operator = ctx.accounts.operator.key();

        if ctx.accounts.banking_settlement_config.is_initialized() {
            ctx.accounts
                .banking_settlement_config
                .change_operator(operator);

            return Ok(());
        }

        ctx.accounts
            .banking_settlement_config
            .initialize(BankingSettlementConfigInitialization {
                operator,
                bump: ctx.bumps.banking_settlement_config,
            });

        Ok(())
    }
}
