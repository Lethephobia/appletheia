use appletheia::aggregate_state;
use appletheia::unique_constraints;

use crate::account::{AccountBalance, AccountId};

use super::{TransferId, TransferStateError, TransferStatus};

/// Stores the materialized state of a `Transfer` aggregate.
#[aggregate_state(error = TransferStateError)]
#[unique_constraints()]
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
