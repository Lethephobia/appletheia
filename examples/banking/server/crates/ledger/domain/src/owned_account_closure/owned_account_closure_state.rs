use appletheia::aggregate_state;
use appletheia::reference_indexes;
use appletheia::unique_constraints;

use crate::account::AccountOwner;

use super::{OwnedAccountClosureId, OwnedAccountClosureStateError, OwnedAccountClosureStatus};

/// Stores the materialized state of an `OwnedAccountClosure` aggregate.
#[aggregate_state(error = OwnedAccountClosureStateError)]
#[unique_constraints()]
#[reference_indexes()]
pub struct OwnedAccountClosureState {
    pub(super) id: OwnedAccountClosureId,
    pub(super) owner: AccountOwner,
    pub(super) closed_account_count: u32,
    pub(super) rejected_account_count: u32,
    pub(super) status: OwnedAccountClosureStatus,
}

impl OwnedAccountClosureState {
    /// Creates a new owned account closure state.
    pub(super) fn new(id: OwnedAccountClosureId, owner: AccountOwner) -> Self {
        Self {
            id,
            owner,
            closed_account_count: 0,
            rejected_account_count: 0,
            status: OwnedAccountClosureStatus::Requested,
        }
    }

    pub(super) fn closed_account_count(&self) -> u32 {
        self.closed_account_count
    }

    pub(super) fn rejected_account_count(&self) -> u32 {
        self.rejected_account_count
    }
}
