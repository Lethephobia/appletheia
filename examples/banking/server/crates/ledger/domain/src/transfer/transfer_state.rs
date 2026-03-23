use appletheia::aggregate_state;
use appletheia::domain::UniqueConstraints;

use crate::account::{AccountBalance, AccountId};

use super::{TransferId, TransferStateError, TransferStatus};

/// Stores the materialized state of a `Transfer` aggregate.
#[aggregate_state(error = TransferStateError)]
pub struct TransferState {
    id: TransferId,
    from_account_id: AccountId,
    to_account_id: AccountId,
    amount: AccountBalance,
    status: TransferStatus,
}

impl TransferState {
    /// Creates a new transfer state.
    pub fn new(
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

    /// Returns the source account.
    pub fn from_account_id(&self) -> &AccountId {
        &self.from_account_id
    }

    /// Returns the destination account.
    pub fn to_account_id(&self) -> &AccountId {
        &self.to_account_id
    }

    /// Returns the transfer amount.
    pub fn amount(&self) -> &AccountBalance {
        &self.amount
    }

    /// Returns the current transfer status.
    pub fn status(&self) -> &TransferStatus {
        &self.status
    }

    /// Marks the transfer as completed.
    pub fn mark_completed(&mut self) {
        self.status = TransferStatus::Completed;
    }

    /// Marks the transfer as failed.
    pub fn mark_failed(&mut self) {
        self.status = TransferStatus::Failed;
    }

    /// Marks the transfer as cancelled.
    pub fn cancel(&mut self) {
        self.status = TransferStatus::Cancelled;
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
        assert_eq!(state.status(), &TransferStatus::Pending);
    }
}
