use appletheia::command;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::{CurrencyAmount, TokenOwnerAddress};
use banking_ledger_domain::token_binding::TokenBindingId;
use banking_ledger_domain::withdrawal::WithdrawalNote;
use serde::{Deserialize, Serialize};

/// Requests a withdrawal from an account to a token account owner address.
#[command(name = "withdrawal_request")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawalRequestCommand {
    pub account_id: AccountId,
    pub token_binding_id: TokenBindingId,
    pub token_owner_address: TokenOwnerAddress,
    pub amount: CurrencyAmount,
    pub note: Option<WithdrawalNote>,
}
