use appletheia::application::saga::SagaState;
use banking_ledger_domain::account::{AccountId, AccountOwner};
use banking_ledger_domain::owned_account_closure::OwnedAccountClosureId;
use serde::{Deserialize, Serialize};

/// Stores data needed by the owned account closure saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedAccountClosureSagaState {
    pub owned_account_closure_id: Option<OwnedAccountClosureId>,
    pub owner: AccountOwner,
    pub next_cursor: Option<AccountId>,
    pub pending_account_ids: Vec<AccountId>,
    pub closed_account_count: u32,
    pub rejected_account_count: u32,
}

impl OwnedAccountClosureSagaState {
    pub fn new(owner: AccountOwner) -> Self {
        Self {
            owned_account_closure_id: None,
            owner,
            next_cursor: None,
            pending_account_ids: Vec::new(),
            closed_account_count: 0,
            rejected_account_count: 0,
        }
    }

    pub fn set_loaded_page(&mut self, account_ids: Vec<AccountId>, next_cursor: Option<AccountId>) {
        self.pending_account_ids = account_ids;
        self.next_cursor = next_cursor;
    }

    pub fn remove_pending_account(&mut self, account_id: AccountId) {
        self.pending_account_ids
            .retain(|pending_account_id| *pending_account_id != account_id);
    }

    pub fn has_pending_accounts(&self) -> bool {
        !self.pending_account_ids.is_empty()
    }

    pub fn has_next_page(&self) -> bool {
        self.next_cursor.is_some()
    }

    pub fn has_rejections(&self) -> bool {
        self.rejected_account_count > 0
    }
}

impl SagaState for OwnedAccountClosureSagaState {}
