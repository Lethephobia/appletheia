use appletheia::command;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

/// Requests eventual creation of the on-chain mint account linked to a currency.
#[command(name = "currency_mint_account_request")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyMintAccountRequestCommand {
    pub currency_id: CurrencyId,
}
