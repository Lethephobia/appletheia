use appletheia::event_payload;

use super::{OwnedAccountClosureCount, OwnedAccountClosureEventPayloadError};
use crate::account::{AccountId, AccountOwner};

#[event_payload(error = OwnedAccountClosureEventPayloadError)]
pub enum OwnedAccountClosureEventPayload {
    Started {
        owner: AccountOwner,
    },
    Requested {
        account_id: AccountId,
    },
    Scanned {
        next_cursor: Option<AccountId>,
    },
    Succeeded {
        account_id: AccountId,
    },
    Failed {
        account_id: AccountId,
    },
    Completed {
        succeeded_count: OwnedAccountClosureCount,
        failed_count: OwnedAccountClosureCount,
    },
}
