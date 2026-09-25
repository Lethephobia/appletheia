use appletheia::application::read_model::{
    ReadModelFragment, ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
};
use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::{
    ChainNetwork, CurrencyAmount, OnchainTransactionId, TokenAddress,
};
use banking_ledger_domain::token_binding::TokenBindingId;
use banking_ledger_domain::transfer::TransferId;
use serde::{Deserialize, Serialize};

use super::{
    AccountTransactionDirection, AccountTransactionFragmentKind, AccountTransactionId,
    AccountTransactionStatus, TransactionNote,
};

/// Normalized account transaction fragment shared by read models.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AccountTransactionFragment {
    pub transaction_id: AccountTransactionId,
    pub transfer_id: Option<TransferId>,
    pub account_id: AccountId,
    pub counterparty_account_id: Option<AccountId>,
    pub token_binding_id: Option<TokenBindingId>,
    pub chain_network: Option<ChainNetwork>,
    pub token_address: Option<TokenAddress>,
    pub onchain_transaction_id: Option<OnchainTransactionId>,
    pub amount: CurrencyAmount,
    pub note: Option<TransactionNote>,
    pub direction: AccountTransactionDirection,
    pub kind: AccountTransactionFragmentKind,
    pub status: AccountTransactionStatus,
    pub occurred_at: EventOccurredAt,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl ReadModelObservationSource for AccountTransactionFragment {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation]
    }
}

impl ReadModelFragment for AccountTransactionFragment {
    const NAME: ReadModelFragmentName = ReadModelFragmentName::new("account_transaction_fragment");

    type Key = AccountTransactionId;

    fn key(&self) -> Self::Key {
        self.transaction_id
    }
}
