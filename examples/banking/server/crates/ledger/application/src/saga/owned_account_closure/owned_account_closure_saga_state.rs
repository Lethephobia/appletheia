use appletheia::application::saga::SagaState;
use banking_ledger_domain::owned_account_closure::OwnedAccountClosureId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedAccountClosureSagaState {
    pub owned_account_closure_id: OwnedAccountClosureId,
}

impl SagaState for OwnedAccountClosureSagaState {}
