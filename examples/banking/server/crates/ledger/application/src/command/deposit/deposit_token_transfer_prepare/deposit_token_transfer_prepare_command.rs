use appletheia::command;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::{CurrencyAmount, TokenAccountOwnerAddress};
use serde::{Deserialize, Serialize};

/// Prepares the external token transfer transaction for a deposit.
#[command(name = "deposit_token_transfer_prepare")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositTokenTransferPrepareCommand {
    pub account_id: AccountId,
    pub token_account_owner_address: TokenAccountOwnerAddress,
    pub amount: CurrencyAmount,
}
