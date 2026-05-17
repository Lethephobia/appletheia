use appletheia::command;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

/// Creates the on-chain mint account linked to a currency.
#[command(name = "currency_mint_account_create")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyMintAccountCreateCommand {
    pub currency_id: CurrencyId,
}
