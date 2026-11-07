use anchor_lang::prelude::*;

use crate::account::BankingSettlementConfigInitialization;
use crate::instruction_handler::ConfigureBankingSettlementConfigInstructionAccounts;

pub(crate) fn handle_configure_banking_settlement_config_instruction(
    ctx: Context<ConfigureBankingSettlementConfigInstructionAccounts>,
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
