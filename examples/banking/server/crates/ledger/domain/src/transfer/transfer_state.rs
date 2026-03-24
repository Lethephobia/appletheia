use appletheia::aggregate_state;
use appletheia::domain::UniqueConstraints;

use crate::account::{AccountBalance, AccountId};

use super::{TransferId, TransferStateError, TransferStatus};

/// Stores the materialized state of a `Transfer` aggregate.
#[aggregate_state(error = TransferStateError)]
pub struct TransferState {
    pub(super) id: TransferId,
    pub(super) from_account_id: AccountId,
    pub(super) to_account_id: AccountId,
    pub(super) amount: AccountBalance,
    pub(super) status: TransferStatus,
}

impl TransferState {
    /// Creates a new transfer state.
    pub(super) fn new(
        id: TransferId,
        from_account_id: AccountId,
        to_account_id: AccountId,
        amount: AccountBalance,
    ) -> Self {
        Self {
            id,
            from_account_id,
            to_account_id,
            amount,
            status: TransferStatus::Pending,
        }
    }
}

impl UniqueConstraints<TransferStateError> for TransferState {}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateState;

    use crate::account::{AccountBalance, AccountId};

    use super::{TransferId, TransferState, TransferStatus};

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = TransferId::new();
        let state = TransferState::new(
            id,
            AccountId::new(),
            AccountId::new(),
            AccountBalance::new(1),
        );

        assert_eq!(state.id(), id);
        assert_eq!(state.status, TransferStatus::Pending);
    }
}
