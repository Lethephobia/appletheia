use appletheia::command;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

/// Synchronizes metadata for an existing on-chain currency mint account.
#[command(name = "mint_metadata_sync")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MintMetadataSyncCommand {
    pub currency_id: CurrencyId,
}
