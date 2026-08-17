use serde::{Deserialize, Serialize};

use appletheia::application::read_model::{ReadModelObservation, ReadModelObservationSource};
use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;

use crate::projection::OwnedAccountTransactionListItemCurrencyPart;
use crate::read_model::OwnedAccountTransactionListItemKind;

use super::{
    AccountTransactionDirection, AccountTransactionFragment, AccountTransactionFragmentKind,
    AccountTransactionId, AccountTransactionStatus,
};

/// Read model for one owned account transaction list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OwnedAccountTransactionListItemPart {
    pub transaction_id: AccountTransactionId,
    pub account_id: AccountId,
    pub currency: OwnedAccountTransactionListItemCurrencyPart,
    pub amount: CurrencyAmount,
    pub direction: AccountTransactionDirection,
    pub kind: OwnedAccountTransactionListItemKind,
    pub status: AccountTransactionStatus,
    pub occurred_at: EventOccurredAt,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl From<AccountTransactionFragment> for OwnedAccountTransactionListItemPart {
    fn from(fragment: AccountTransactionFragment) -> Self {
        let kind = match fragment.kind {
            AccountTransactionFragmentKind::Deposit => OwnedAccountTransactionListItemKind::Deposit,
            AccountTransactionFragmentKind::Withdrawal => {
                OwnedAccountTransactionListItemKind::Withdrawal
            }
            AccountTransactionFragmentKind::CurrencyIssuance => {
                OwnedAccountTransactionListItemKind::CurrencyIssuance
            }
            AccountTransactionFragmentKind::Transfer => OwnedAccountTransactionListItemKind::Transfer {
                transfer_id: fragment
                    .transfer_id
                    .expect("materialized transfer transaction must have a transfer id"),
                counterparty_account: Box::new(
                    fragment
                        .counterparty_account
                        .expect("materialized transfer transaction must have a counterparty account")
                        .into(),
                ),
            },
        };

        Self {
            transaction_id: fragment.transaction_id,
            account_id: fragment.account.id,
            currency: fragment.account.currency.into(),
            amount: fragment.amount,
            direction: fragment.direction,
            kind,
            status: fragment.status,
            occurred_at: fragment.occurred_at,
            created_at: fragment.created_at,
            observation: fragment.observation,
        }
    }
}

impl ReadModelObservationSource for OwnedAccountTransactionListItemPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        std::iter::once(self.observation)
            .chain(self.currency.observations())
            .chain(self.kind.observations())
            .collect()
    }
}
