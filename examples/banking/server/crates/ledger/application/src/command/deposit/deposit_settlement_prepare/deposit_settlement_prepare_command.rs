use appletheia::command;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::{CurrencyAmount, TokenOwnerAddress};
use banking_ledger_domain::deposit::DepositNote;
use banking_ledger_domain::token_binding::TokenBindingId;
use serde::{Deserialize, Serialize};

use crate::settlement::EvmDepositAuthorization;

/// Prepares the external token settlement transaction for a deposit.
#[command(name = "deposit_settlement_prepare")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositSettlementPrepareCommand {
    pub account_id: AccountId,
    pub token_binding_id: TokenBindingId,
    pub token_owner_address: TokenOwnerAddress,
    pub amount: CurrencyAmount,
    /// Uses an existing allowance when omitted for an EVM deposit.
    pub evm_authorization: Option<EvmDepositAuthorization>,
    pub note: Option<DepositNote>,
}
