use appletheia::event_payload;

use crate::account::{AccountCloseRejectionReason, AccountId, AccountOwner};

use super::{
    OwnedAccountClosureCompleteRejectionReason, OwnedAccountClosureEventPayloadError,
    OwnedAccountClosureFailRejectionReason, OwnedAccountClosureFailureReason,
    OwnedAccountClosureId, OwnedAccountClosurePageLoadRejectionReason,
    OwnedAccountClosureRecordRejectionReason,
};

/// Represents the domain events emitted by an `OwnedAccountClosure` aggregate.
#[event_payload(error = OwnedAccountClosureEventPayloadError)]
pub enum OwnedAccountClosureEventPayload {
    Requested {
        id: OwnedAccountClosureId,
        owner: AccountOwner,
    },
    PageLoaded {
        account_ids: Vec<AccountId>,
        next_cursor: Option<AccountId>,
    },
    PageLoadRejected {
        reason: OwnedAccountClosurePageLoadRejectionReason,
    },
    AccountCloseRecorded {
        account_id: AccountId,
    },
    AccountCloseRecordRejected {
        account_id: AccountId,
        reason: OwnedAccountClosureRecordRejectionReason,
    },
    AccountCloseRejectionRecorded {
        account_id: AccountId,
        reason: AccountCloseRejectionReason,
    },
    AccountCloseRejectionRecordRejected {
        account_id: AccountId,
        reason: OwnedAccountClosureRecordRejectionReason,
    },
    Completed {
        closed_account_count: u32,
    },
    CompleteRejected {
        reason: OwnedAccountClosureCompleteRejectionReason,
    },
    Failed {
        closed_account_count: u32,
        rejected_account_count: u32,
        reason: OwnedAccountClosureFailureReason,
    },
    FailRejected {
        reason: OwnedAccountClosureFailRejectionReason,
    },
}
