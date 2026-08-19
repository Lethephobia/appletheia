use appletheia::application::read_model::{
    ReadModelFragment, ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
};
use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::transfer::TransferId;
use serde::{Deserialize, Serialize};

use super::{
    AccountTransactionDirection, AccountTransactionFragmentKind, AccountTransactionId,
    AccountTransactionStatus,
};

/// Normalized account transaction fragment shared by read models.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AccountTransactionFragment {
    pub transaction_id: AccountTransactionId,
    pub transfer_id: Option<TransferId>,
    pub account_id: AccountId,
    pub counterparty_account_id: Option<AccountId>,
    pub amount: CurrencyAmount,
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
