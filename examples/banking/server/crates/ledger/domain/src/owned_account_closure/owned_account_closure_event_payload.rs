use appletheia::event_payload;

use super::{OwnedAccountClosureCount, OwnedAccountClosureEventPayloadError};
use crate::account::{AccountId, AccountOwner};

#[event_payload(error = OwnedAccountClosureEventPayloadError)]
pub enum OwnedAccountClosureEventPayload {
    Started {
        owner: AccountOwner,
    },
    Scanned {
        account_ids: Vec<AccountId>,
        next_cursor: Option<AccountId>,
    },
    AccountSucceeded {
        account_id: AccountId,
    },
    AccountFailed {
        account_id: AccountId,
    },
    Completed {
        succeeded_count: OwnedAccountClosureCount,
        failed_count: OwnedAccountClosureCount,
    },
}
