use appletheia::event_payload;

use crate::account::{AccountCloseRejectionReason, AccountId, AccountOwner};

use super::{OwnedAccountClosureEventPayloadError, OwnedAccountClosureFailureReason};

/// Represents the domain events emitted by an `OwnedAccountClosure` aggregate.
#[event_payload(error = OwnedAccountClosureEventPayloadError)]
pub enum OwnedAccountClosureEventPayload {
    Requested {
        owner: AccountOwner,
    },
    PageLoaded {
        account_ids: Vec<AccountId>,
        next_cursor: Option<AccountId>,
    },
    AccountCloseRecorded {
        account_id: AccountId,
    },
    AccountCloseRejectionRecorded {
        account_id: AccountId,
        reason: AccountCloseRejectionReason,
    },
    Completed {
        closed_account_count: u32,
    },
    Failed {
        closed_account_count: u32,
        rejected_account_count: u32,
        reason: OwnedAccountClosureFailureReason,
    },
}
