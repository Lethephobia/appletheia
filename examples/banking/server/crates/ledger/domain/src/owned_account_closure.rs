mod owned_account_closure_count;
mod owned_account_closure_count_error;
mod owned_account_closure_error;
mod owned_account_closure_event_payload;
mod owned_account_closure_event_payload_error;
mod owned_account_closure_id;
mod owned_account_closure_state;
mod owned_account_closure_state_error;
mod owned_account_closure_status;

pub use owned_account_closure_count::OwnedAccountClosureCount;
pub use owned_account_closure_count_error::OwnedAccountClosureCountError;
pub use owned_account_closure_error::OwnedAccountClosureError;
pub use owned_account_closure_event_payload::OwnedAccountClosureEventPayload;
pub use owned_account_closure_event_payload_error::OwnedAccountClosureEventPayloadError;
pub use owned_account_closure_id::OwnedAccountClosureId;
pub use owned_account_closure_state::OwnedAccountClosureState;
pub use owned_account_closure_state_error::OwnedAccountClosureStateError;
pub use owned_account_closure_status::OwnedAccountClosureStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::account::{AccountId, AccountOwner};

/// Tracks scan and outcomes for all accounts owned by a removed owner.
#[aggregate(type = "owned_account_closure", error = OwnedAccountClosureError)]
pub struct OwnedAccountClosure {
    core: AggregateCore<
        OwnedAccountClosureId,
        OwnedAccountClosureState,
        OwnedAccountClosureEventPayload,
    >,
}

impl OwnedAccountClosure {
    pub fn owner(&self) -> Result<AccountOwner, OwnedAccountClosureError> {
        Ok(self.state_required()?.owner)
    }

    pub fn status(&self) -> Result<OwnedAccountClosureStatus, OwnedAccountClosureError> {
        Ok(self.state_required()?.status)
    }

    pub fn next_cursor(&self) -> Result<Option<AccountId>, OwnedAccountClosureError> {
        Ok(self.state_required()?.next_cursor)
    }

    pub fn is_scan_completed(&self) -> Result<bool, OwnedAccountClosureError> {
        Ok(self.state_required()?.scan_completed)
    }

    pub fn is_ready_to_complete(&self) -> Result<bool, OwnedAccountClosureError> {
        let state = self.state_required()?;
        Ok(state.status == OwnedAccountClosureStatus::InProgress
            && state.scan_completed
            && state.requested_count
                == state
                    .succeeded_count
                    .try_add(state.failed_count.value() as usize)?)
    }

    pub fn start(&mut self, owner: AccountOwner) -> Result<(), OwnedAccountClosureError> {
        if self.state().is_some() {
            return Err(OwnedAccountClosureError::AlreadyStarted);
        }
        self.append_event(OwnedAccountClosureEventPayload::Started { owner })?;
        Ok(())
    }

    pub fn record_scanned(
        &mut self,
        account_ids: Vec<AccountId>,
        next_cursor: Option<AccountId>,
    ) -> Result<(), OwnedAccountClosureError> {
        let state = self.state_required()?;
        if state.status == OwnedAccountClosureStatus::Completed {
            return Err(OwnedAccountClosureError::AlreadyCompleted);
        }
        if state.scan_completed {
            return Err(OwnedAccountClosureError::ScanAlreadyCompleted);
        }
        if next_cursor.is_some_and(|next| state.next_cursor.is_some_and(|cursor| next <= cursor)) {
            return Err(OwnedAccountClosureError::UnexpectedCursor);
        }
        state.requested_count.try_add(account_ids.len())?;
        self.append_event(OwnedAccountClosureEventPayload::Scanned {
            account_ids,
            next_cursor,
        })?;
        Ok(())
    }

    pub fn record_account_succeeded(
        &mut self,
        account_id: AccountId,
    ) -> Result<(), OwnedAccountClosureError> {
        let state = self.state_required()?;
        if state.status == OwnedAccountClosureStatus::Completed {
            return Err(OwnedAccountClosureError::AlreadyCompleted);
        }
        if state
            .succeeded_count
            .try_add(state.failed_count.value() as usize)?
            >= state.requested_count
        {
            return Err(OwnedAccountClosureError::NoPendingResults);
        }
        self.append_event(OwnedAccountClosureEventPayload::AccountSucceeded { account_id })?;
        Ok(())
    }

    pub fn record_account_failed(
        &mut self,
        account_id: AccountId,
    ) -> Result<(), OwnedAccountClosureError> {
        let state = self.state_required()?;
        if state.status == OwnedAccountClosureStatus::Completed {
            return Err(OwnedAccountClosureError::AlreadyCompleted);
        }
        if state
            .succeeded_count
            .try_add(state.failed_count.value() as usize)?
            >= state.requested_count
        {
            return Err(OwnedAccountClosureError::NoPendingResults);
        }
        self.append_event(OwnedAccountClosureEventPayload::AccountFailed { account_id })?;
        Ok(())
    }

    pub fn complete(&mut self) -> Result<(), OwnedAccountClosureError> {
        if self.state_required()?.status == OwnedAccountClosureStatus::Completed {
            return Err(OwnedAccountClosureError::AlreadyCompleted);
        }
        if !self.is_ready_to_complete()? {
            return Err(OwnedAccountClosureError::NotReadyToComplete);
        }
        let state = self.state_required()?;
        self.append_event(OwnedAccountClosureEventPayload::Completed {
            succeeded_count: state.succeeded_count,
            failed_count: state.failed_count,
        })?;
        Ok(())
    }
}

impl AggregateApply<OwnedAccountClosureEventPayload, OwnedAccountClosureError>
    for OwnedAccountClosure
{
    fn apply(
        &mut self,
        payload: &OwnedAccountClosureEventPayload,
    ) -> Result<(), OwnedAccountClosureError> {
        match payload {
            OwnedAccountClosureEventPayload::Started { owner } => {
                self.set_state(Some(OwnedAccountClosureState {
                    owner: *owner,
                    requested_count: OwnedAccountClosureCount::default(),
                    succeeded_count: OwnedAccountClosureCount::default(),
                    failed_count: OwnedAccountClosureCount::default(),
                    next_cursor: None,
                    scan_completed: false,
                    status: OwnedAccountClosureStatus::InProgress,
                }));
            }
            OwnedAccountClosureEventPayload::Scanned {
                account_ids,
                next_cursor,
            } => {
                let state = self.state_required_mut()?;
                state.requested_count = state.requested_count.try_add(account_ids.len())?;
                state.next_cursor = *next_cursor;
                state.scan_completed = next_cursor.is_none();
            }
            OwnedAccountClosureEventPayload::AccountSucceeded { .. } => {
                let state = self.state_required_mut()?;
                state.succeeded_count = state.succeeded_count.try_increment()?;
            }
            OwnedAccountClosureEventPayload::AccountFailed { .. } => {
                let state = self.state_required_mut()?;
                state.failed_count = state.failed_count.try_increment()?;
            }
            OwnedAccountClosureEventPayload::Completed { .. } => {
                self.state_required_mut()?.status = OwnedAccountClosureStatus::Completed;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use banking_iam_domain::UserId;

    fn started() -> OwnedAccountClosure {
        let mut closure = OwnedAccountClosure::new();
        closure.start(AccountOwner::User(UserId::new())).unwrap();
        closure
    }

    fn account_ids() -> [AccountId; 2] {
        let mut ids = [AccountId::new(), AccountId::new()];
        ids.sort();
        ids
    }

    #[test]
    fn each_operation_emits_only_its_own_event() {
        let mut closure = started();
        let [first, second] = account_ids();
        closure.record_scanned(vec![first], Some(first)).unwrap();
        closure.record_scanned(vec![second], None).unwrap();
        closure.record_account_succeeded(first).unwrap();
        closure.record_account_failed(second).unwrap();
        assert_eq!(closure.uncommitted_events().len(), 5);
        assert!(closure.is_ready_to_complete().unwrap());
        assert_eq!(
            closure.status().unwrap(),
            OwnedAccountClosureStatus::InProgress
        );
        closure.complete().unwrap();
        assert_eq!(closure.uncommitted_events().len(), 6);
        let payloads: Vec<_> = closure
            .uncommitted_events()
            .iter()
            .map(|event| event.payload())
            .collect();
        assert!(matches!(
            payloads.as_slice(),
            [
                OwnedAccountClosureEventPayload::Started { .. },
                OwnedAccountClosureEventPayload::Scanned { .. },
                OwnedAccountClosureEventPayload::Scanned { next_cursor: None, .. },
                OwnedAccountClosureEventPayload::AccountSucceeded { .. },
                OwnedAccountClosureEventPayload::AccountFailed { .. },
                OwnedAccountClosureEventPayload::Completed {
                    succeeded_count,
                    failed_count
                },
            ] if succeeded_count.value() == 1 && failed_count.value() == 1
        ));
    }

    #[test]
    fn empty_scan_requires_explicit_completion() {
        let mut closure = started();
        assert!(!closure.is_ready_to_complete().unwrap());
        assert!(closure.complete().is_err());
        closure.record_scanned(vec![], None).unwrap();
        assert_eq!(closure.uncommitted_events().len(), 2);
        assert!(closure.is_ready_to_complete().unwrap());
        closure.complete().unwrap();
        assert_eq!(
            closure.status().unwrap(),
            OwnedAccountClosureStatus::Completed
        );
        assert!(!closure.is_ready_to_complete().unwrap());
        assert!(closure.complete().is_err());
        assert_eq!(closure.uncommitted_events().len(), 3);
    }

    #[test]
    fn readiness_requires_both_scan_completion_and_all_results() {
        for scan_finishes_first in [false, true] {
            let mut closure = started();
            let account_id = AccountId::new();
            closure
                .record_scanned(vec![account_id], Some(account_id))
                .unwrap();
            if scan_finishes_first {
                closure.record_scanned(vec![], None).unwrap();
            } else {
                closure.record_account_succeeded(account_id).unwrap();
            }
            assert!(!closure.is_ready_to_complete().unwrap());
            let count = closure.uncommitted_events().len();
            assert!(matches!(
                closure.complete(),
                Err(OwnedAccountClosureError::NotReadyToComplete)
            ));
            assert_eq!(closure.uncommitted_events().len(), count);
            if scan_finishes_first {
                closure.record_account_succeeded(account_id).unwrap();
            } else {
                closure.record_scanned(vec![], None).unwrap();
            }
            assert!(closure.is_ready_to_complete().unwrap());
        }
    }

    #[test]
    fn result_count_cannot_exceed_requested_count() {
        let mut closure = started();
        let account_id = AccountId::new();
        assert!(matches!(
            closure.record_account_succeeded(account_id),
            Err(OwnedAccountClosureError::NoPendingResults)
        ));
        assert!(matches!(
            closure.record_account_failed(account_id),
            Err(OwnedAccountClosureError::NoPendingResults)
        ));
        closure
            .record_scanned(vec![account_id], Some(account_id))
            .unwrap();
        closure.record_account_succeeded(account_id).unwrap();
        let count = closure.uncommitted_events().len();
        assert!(matches!(
            closure.record_account_failed(account_id),
            Err(OwnedAccountClosureError::NoPendingResults)
        ));
        assert!(matches!(
            closure.record_account_succeeded(account_id),
            Err(OwnedAccountClosureError::NoPendingResults)
        ));
        assert_eq!(closure.uncommitted_events().len(), count);
    }

    #[test]
    fn scan_cursor_advances_without_retaining_requested_account_ids() {
        let mut closure = started();
        let [first, second] = account_ids();
        closure.record_scanned(vec![first], Some(first)).unwrap();
        assert!(closure.record_scanned(vec![], Some(first)).is_err());
        closure.record_scanned(vec![second], Some(second)).unwrap();
        assert_eq!(closure.next_cursor().unwrap(), Some(second));
        closure.record_scanned(vec![], None).unwrap();
        assert!(
            closure
                .record_scanned(vec![AccountId::new()], None)
                .is_err()
        );
        assert!(closure.record_scanned(vec![], None).is_err());
        let state = serde_json::to_value(closure.state_required().unwrap()).unwrap();
        assert_eq!(state["requested_count"], 2);
        assert!(state.get("pending_account_ids").is_none());
        assert!(state.get("last_requested_account_id").is_none());
    }

    #[test]
    fn scan_count_overflow_is_rejected_before_appending_an_event() {
        let mut closure = started();
        closure.state_required_mut().unwrap().requested_count =
            OwnedAccountClosureCount::new(u32::MAX);
        assert!(matches!(
            closure.record_scanned(vec![AccountId::new()], None),
            Err(OwnedAccountClosureError::Count(
                OwnedAccountClosureCountError::Overflow
            ))
        ));
        assert_eq!(closure.uncommitted_events().len(), 1);
    }

    #[test]
    fn replay_preserves_counts_without_emitting_completion() {
        let mut closure = started();
        let [first, second] = account_ids();
        closure.record_scanned(vec![first, second], None).unwrap();
        closure.record_account_failed(first).unwrap();
        let mut replayed = OwnedAccountClosure::new();
        for event in closure.uncommitted_events() {
            replayed.apply(event.payload()).unwrap();
        }
        assert!(replayed.uncommitted_events().is_empty());
        replayed.record_account_succeeded(second).unwrap();
        assert_eq!(replayed.uncommitted_events().len(), 1);
        assert!(replayed.is_ready_to_complete().unwrap());
        replayed.complete().unwrap();
        assert!(matches!(
            replayed.uncommitted_events().last().unwrap().payload(),
            OwnedAccountClosureEventPayload::Completed {
                succeeded_count, failed_count
            } if succeeded_count.value() == 1 && failed_count.value() == 1
        ));
    }
}
