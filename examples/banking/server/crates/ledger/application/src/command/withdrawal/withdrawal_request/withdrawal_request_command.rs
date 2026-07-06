use appletheia::command;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::{CurrencyAmount, TokenAccountOwnerAddress};
use serde::{Deserialize, Serialize};

/// Requests a withdrawal from an account to a token account owner address.
#[command(name = "withdrawal_request")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawalRequestCommand {
    pub account_id: AccountId,
    pub token_account_owner_address: TokenAccountOwnerAddress,
    pub amount: CurrencyAmount,
}
