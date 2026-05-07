use appletheia::aggregate_state;
use appletheia::reference_indexes;
use appletheia::unique_constraints;

use crate::account::AccountId;
use crate::core::CurrencyAmount;

use super::{TransferId, TransferStateError, TransferStatus};

/// Stores the materialized state of a `Transfer` aggregate.
#[aggregate_state(error = TransferStateError)]
#[unique_constraints()]
#[reference_indexes(
    entry(key = "from_account", value = from_account_value),
    entry(key = "to_account", value = to_account_value)
)]
pub struct TransferState {
    pub(super) id: TransferId,
    pub(super) from_account_id: AccountId,
    pub(super) to_account_id: AccountId,
    pub(super) amount: CurrencyAmount,
    pub(super) status: TransferStatus,
}

impl TransferState {
    /// Creates a new transfer state.
    pub(super) fn new(
        id: TransferId,
        from_account_id: AccountId,
        to_account_id: AccountId,
        amount: CurrencyAmount,
    ) -> Self {
        Self {
            id,
            from_account_id,
            to_account_id,
            amount,
            status: TransferStatus::Pending,
        }
    }

    /// Creates a new rejected transfer state.
    pub(super) fn rejected(
        id: TransferId,
        from_account_id: AccountId,
        to_account_id: AccountId,
        amount: CurrencyAmount,
    ) -> Self {
        Self {
            id,
            from_account_id,
            to_account_id,
            amount,
            status: TransferStatus::Rejected,
        }
    }
}

fn from_account_value(state: &TransferState) -> Result<Option<AccountId>, TransferStateError> {
    Ok(Some(state.from_account_id))
}

fn to_account_value(state: &TransferState) -> Result<Option<AccountId>, TransferStateError> {
    Ok(Some(state.to_account_id))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{AggregateState, ReferenceIndexes, ReferenceValues};

    use crate::account::AccountId;
    use crate::core::CurrencyAmount;

    use super::{TransferId, TransferState, TransferStatus};

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = TransferId::new();
        let state = TransferState::new(
            id,
            AccountId::new(),
            AccountId::new(),
            CurrencyAmount::new(1),
        );

        assert_eq!(state.id(), id);
        assert_eq!(state.status, TransferStatus::Pending);
    }

    #[test]
    fn returns_reference_entries_for_source_and_destination_accounts() {
        let state = TransferState::new(
            TransferId::new(),
            AccountId::new(),
            AccountId::new(),
            CurrencyAmount::new(1),
        );

        let entries = state
            .reference_entries()
            .expect("reference entries should build");

        assert_eq!(
            entries
                .get(TransferState::FROM_ACCOUNT_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
        assert_eq!(
            entries
                .get(TransferState::TO_ACCOUNT_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
    }
}
