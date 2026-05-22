use appletheia::command;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

/// Synchronizes metadata for an existing on-chain currency mint account.
#[command(name = "currency_mint_account_metadata_sync")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyMintAccountMetadataSyncCommand {
    pub currency_id: CurrencyId,
}
