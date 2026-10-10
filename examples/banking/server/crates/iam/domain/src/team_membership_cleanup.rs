mod team_membership_cleanup_count;
mod team_membership_cleanup_count_error;
mod team_membership_cleanup_error;
mod team_membership_cleanup_event_payload;
mod team_membership_cleanup_event_payload_error;
mod team_membership_cleanup_id;
mod team_membership_cleanup_state;
mod team_membership_cleanup_state_error;
mod team_membership_cleanup_status;

pub use team_membership_cleanup_count::TeamMembershipCleanupCount;
pub use team_membership_cleanup_count_error::TeamMembershipCleanupCountError;
pub use team_membership_cleanup_error::TeamMembershipCleanupError;
pub use team_membership_cleanup_event_payload::TeamMembershipCleanupEventPayload;
pub use team_membership_cleanup_event_payload_error::TeamMembershipCleanupEventPayloadError;
pub use team_membership_cleanup_id::TeamMembershipCleanupId;
pub use team_membership_cleanup_state::TeamMembershipCleanupState;
pub use team_membership_cleanup_state_error::TeamMembershipCleanupStateError;
pub use team_membership_cleanup_status::TeamMembershipCleanupStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::{OrganizationMembershipId, TeamMembershipId};

#[aggregate(type = "team_membership_cleanup", error = TeamMembershipCleanupError)]
pub struct TeamMembershipCleanup {
    core: AggregateCore<
        TeamMembershipCleanupId,
        TeamMembershipCleanupState,
        TeamMembershipCleanupEventPayload,
    >,
}

impl TeamMembershipCleanup {
    pub fn organization_membership_id(
        &self,
    ) -> Result<OrganizationMembershipId, TeamMembershipCleanupError> {
        Ok(self.state_required()?.organization_membership_id)
    }

    pub fn status(&self) -> Result<TeamMembershipCleanupStatus, TeamMembershipCleanupError> {
        Ok(self.state_required()?.status)
    }

    pub fn next_cursor(&self) -> Result<Option<TeamMembershipId>, TeamMembershipCleanupError> {
        Ok(self.state_required()?.next_cursor)
    }

    pub fn is_scan_completed(&self) -> Result<bool, TeamMembershipCleanupError> {
        Ok(self.state_required()?.scan_completed)
    }

    pub fn is_ready_to_complete(&self) -> Result<bool, TeamMembershipCleanupError> {
        let state = self.state_required()?;
        Ok(state.status == TeamMembershipCleanupStatus::InProgress
            && state.scan_completed
            && state.requested_count
                == state
                    .succeeded_count
                    .try_add(state.failed_count.value() as usize)?)
    }

    pub fn start(
        &mut self,
        organization_membership_id: OrganizationMembershipId,
    ) -> Result<(), TeamMembershipCleanupError> {
        if self.state().is_some() {
            return Err(TeamMembershipCleanupError::AlreadyStarted);
        }
        self.append_event(TeamMembershipCleanupEventPayload::Started {
            organization_membership_id,
        })?;
        Ok(())
    }

    pub fn record_scanned(
        &mut self,
        team_membership_ids: Vec<TeamMembershipId>,
        next_cursor: Option<TeamMembershipId>,
    ) -> Result<(), TeamMembershipCleanupError> {
        let state = self.state_required()?;
        if state.status == TeamMembershipCleanupStatus::Completed {
            return Err(TeamMembershipCleanupError::AlreadyCompleted);
        }
        if state.scan_completed {
            return Err(TeamMembershipCleanupError::ScanAlreadyCompleted);
        }
        if next_cursor.is_some_and(|next| state.next_cursor.is_some_and(|cursor| next <= cursor)) {
            return Err(TeamMembershipCleanupError::UnexpectedCursor);
        }
        state.requested_count.try_add(team_membership_ids.len())?;
        self.append_event(TeamMembershipCleanupEventPayload::Scanned {
            team_membership_ids,
            next_cursor,
        })?;
        Ok(())
    }

    pub fn record_membership_succeeded(
        &mut self,
        team_membership_id: TeamMembershipId,
    ) -> Result<(), TeamMembershipCleanupError> {
        let state = self.state_required()?;
        if state.status == TeamMembershipCleanupStatus::Completed {
            return Err(TeamMembershipCleanupError::AlreadyCompleted);
        }
        if state
            .succeeded_count
            .try_add(state.failed_count.value() as usize)?
            >= state.requested_count
        {
            return Err(TeamMembershipCleanupError::NoPendingResults);
        }
        self.append_event(TeamMembershipCleanupEventPayload::MembershipSucceeded {
            team_membership_id,
        })?;
        Ok(())
    }

    pub fn record_membership_failed(
        &mut self,
        team_membership_id: TeamMembershipId,
    ) -> Result<(), TeamMembershipCleanupError> {
        let state = self.state_required()?;
        if state.status == TeamMembershipCleanupStatus::Completed {
            return Err(TeamMembershipCleanupError::AlreadyCompleted);
        }
        if state
            .succeeded_count
            .try_add(state.failed_count.value() as usize)?
            >= state.requested_count
        {
            return Err(TeamMembershipCleanupError::NoPendingResults);
        }
        self.append_event(TeamMembershipCleanupEventPayload::MembershipFailed {
            team_membership_id,
        })?;
        Ok(())
    }

    pub fn complete(&mut self) -> Result<(), TeamMembershipCleanupError> {
        if self.state_required()?.status == TeamMembershipCleanupStatus::Completed {
            return Err(TeamMembershipCleanupError::AlreadyCompleted);
        }
        if !self.is_ready_to_complete()? {
            return Err(TeamMembershipCleanupError::NotReadyToComplete);
        }
        let state = self.state_required()?;
        self.append_event(TeamMembershipCleanupEventPayload::Completed {
            succeeded_count: state.succeeded_count,
            failed_count: state.failed_count,
        })?;
        Ok(())
    }
}

impl AggregateApply<TeamMembershipCleanupEventPayload, TeamMembershipCleanupError>
    for TeamMembershipCleanup
{
    fn apply(
        &mut self,
        payload: &TeamMembershipCleanupEventPayload,
    ) -> Result<(), TeamMembershipCleanupError> {
        match payload {
            TeamMembershipCleanupEventPayload::Started {
                organization_membership_id,
            } => {
                self.set_state(Some(TeamMembershipCleanupState {
                    organization_membership_id: *organization_membership_id,
                    requested_count: TeamMembershipCleanupCount::default(),
                    succeeded_count: TeamMembershipCleanupCount::default(),
                    failed_count: TeamMembershipCleanupCount::default(),
                    next_cursor: None,
                    scan_completed: false,
                    status: TeamMembershipCleanupStatus::InProgress,
                }));
            }
            TeamMembershipCleanupEventPayload::Scanned {
                team_membership_ids,
                next_cursor,
            } => {
                let state = self.state_required_mut()?;
                state.requested_count = state.requested_count.try_add(team_membership_ids.len())?;
                state.next_cursor = *next_cursor;
                state.scan_completed = next_cursor.is_none();
            }
            TeamMembershipCleanupEventPayload::MembershipSucceeded { .. } => {
                let state = self.state_required_mut()?;
                state.succeeded_count = state.succeeded_count.try_increment()?;
            }
            TeamMembershipCleanupEventPayload::MembershipFailed { .. } => {
                let state = self.state_required_mut()?;
                state.failed_count = state.failed_count.try_increment()?;
            }
            TeamMembershipCleanupEventPayload::Completed { .. } => {
                self.state_required_mut()?.status = TeamMembershipCleanupStatus::Completed;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::OrganizationMembershipId;

    fn started() -> TeamMembershipCleanup {
        let mut cleanup = TeamMembershipCleanup::new();
        cleanup.start(OrganizationMembershipId::new()).unwrap();
        cleanup
    }

    fn team_membership_ids() -> [TeamMembershipId; 2] {
        let mut ids = [TeamMembershipId::new(), TeamMembershipId::new()];
        ids.sort();
        ids
    }

    #[test]
    fn each_operation_emits_only_its_own_event() {
        let mut cleanup = started();
        let [first, second] = team_membership_ids();
        cleanup.record_scanned(vec![first], Some(first)).unwrap();
        cleanup.record_scanned(vec![second], None).unwrap();
        cleanup.record_membership_succeeded(first).unwrap();
        cleanup.record_membership_failed(second).unwrap();
        assert_eq!(cleanup.uncommitted_events().len(), 5);
        assert!(cleanup.is_ready_to_complete().unwrap());
        assert_eq!(
            cleanup.status().unwrap(),
            TeamMembershipCleanupStatus::InProgress
        );
        cleanup.complete().unwrap();
        assert_eq!(cleanup.uncommitted_events().len(), 6);
        let payloads: Vec<_> = cleanup
            .uncommitted_events()
            .iter()
            .map(|event| event.payload())
            .collect();
        assert!(matches!(
            payloads.as_slice(),
            [
                TeamMembershipCleanupEventPayload::Started { .. },
                TeamMembershipCleanupEventPayload::Scanned { .. },
                TeamMembershipCleanupEventPayload::Scanned { next_cursor: None, .. },
                TeamMembershipCleanupEventPayload::MembershipSucceeded { .. },
                TeamMembershipCleanupEventPayload::MembershipFailed { .. },
                TeamMembershipCleanupEventPayload::Completed {
                    succeeded_count,
                    failed_count
                },
            ] if succeeded_count.value() == 1 && failed_count.value() == 1
        ));
    }

    #[test]
    fn empty_scan_requires_explicit_completion() {
        let mut cleanup = started();
        assert!(!cleanup.is_ready_to_complete().unwrap());
        assert!(cleanup.complete().is_err());
        cleanup.record_scanned(vec![], None).unwrap();
        assert_eq!(cleanup.uncommitted_events().len(), 2);
        assert!(cleanup.is_ready_to_complete().unwrap());
        cleanup.complete().unwrap();
        assert_eq!(
            cleanup.status().unwrap(),
            TeamMembershipCleanupStatus::Completed
        );
        assert!(!cleanup.is_ready_to_complete().unwrap());
        assert!(cleanup.complete().is_err());
        assert_eq!(cleanup.uncommitted_events().len(), 3);
    }

    #[test]
    fn readiness_requires_both_scan_completion_and_all_results() {
        for scan_finishes_first in [false, true] {
            let mut cleanup = started();
            let team_membership_id = TeamMembershipId::new();
            cleanup
                .record_scanned(vec![team_membership_id], Some(team_membership_id))
                .unwrap();
            if scan_finishes_first {
                cleanup.record_scanned(vec![], None).unwrap();
            } else {
                cleanup
                    .record_membership_succeeded(team_membership_id)
                    .unwrap();
            }
            assert!(!cleanup.is_ready_to_complete().unwrap());
            let count = cleanup.uncommitted_events().len();
            assert!(matches!(
                cleanup.complete(),
                Err(TeamMembershipCleanupError::NotReadyToComplete)
            ));
            assert_eq!(cleanup.uncommitted_events().len(), count);
            if scan_finishes_first {
                cleanup
                    .record_membership_succeeded(team_membership_id)
                    .unwrap();
            } else {
                cleanup.record_scanned(vec![], None).unwrap();
            }
            assert!(cleanup.is_ready_to_complete().unwrap());
        }
    }

    #[test]
    fn result_count_cannot_exceed_requested_count() {
        let mut cleanup = started();
        let team_membership_id = TeamMembershipId::new();
        assert!(matches!(
            cleanup.record_membership_succeeded(team_membership_id),
            Err(TeamMembershipCleanupError::NoPendingResults)
        ));
        assert!(matches!(
            cleanup.record_membership_failed(team_membership_id),
            Err(TeamMembershipCleanupError::NoPendingResults)
        ));
        cleanup
            .record_scanned(vec![team_membership_id], Some(team_membership_id))
            .unwrap();
        cleanup
            .record_membership_succeeded(team_membership_id)
            .unwrap();
        let count = cleanup.uncommitted_events().len();
        assert!(matches!(
            cleanup.record_membership_failed(team_membership_id),
            Err(TeamMembershipCleanupError::NoPendingResults)
        ));
        assert!(matches!(
            cleanup.record_membership_succeeded(team_membership_id),
            Err(TeamMembershipCleanupError::NoPendingResults)
        ));
        assert_eq!(cleanup.uncommitted_events().len(), count);
    }

    #[test]
    fn scan_cursor_advances_without_retaining_requested_team_membership_ids() {
        let mut cleanup = started();
        let [first, second] = team_membership_ids();
        cleanup.record_scanned(vec![first], Some(first)).unwrap();
        assert!(cleanup.record_scanned(vec![], Some(first)).is_err());
        cleanup.record_scanned(vec![second], Some(second)).unwrap();
        assert_eq!(cleanup.next_cursor().unwrap(), Some(second));
        cleanup.record_scanned(vec![], None).unwrap();
        assert!(
            cleanup
                .record_scanned(vec![TeamMembershipId::new()], None)
                .is_err()
        );
        assert!(cleanup.record_scanned(vec![], None).is_err());
        let state = serde_json::to_value(cleanup.state_required().unwrap()).unwrap();
        assert_eq!(state["requested_count"], 2);
        assert!(state.get("pending_team_membership_ids").is_none());
        assert!(state.get("last_requested_team_membership_id").is_none());
    }

    #[test]
    fn scan_count_overflow_is_rejected_before_appending_an_event() {
        let mut cleanup = started();
        cleanup.state_required_mut().unwrap().requested_count =
            TeamMembershipCleanupCount::new(u32::MAX);
        assert!(matches!(
            cleanup.record_scanned(vec![TeamMembershipId::new()], None),
            Err(TeamMembershipCleanupError::Count(
                TeamMembershipCleanupCountError::Overflow
            ))
        ));
        assert_eq!(cleanup.uncommitted_events().len(), 1);
    }

    #[test]
    fn replay_preserves_counts_without_emitting_completion() {
        let mut cleanup = started();
        let [first, second] = team_membership_ids();
        cleanup.record_scanned(vec![first, second], None).unwrap();
        cleanup.record_membership_failed(first).unwrap();
        let mut replayed = TeamMembershipCleanup::new();
        for event in cleanup.uncommitted_events() {
            replayed.apply(event.payload()).unwrap();
        }
        assert!(replayed.uncommitted_events().is_empty());
        replayed.record_membership_succeeded(second).unwrap();
        assert_eq!(replayed.uncommitted_events().len(), 1);
        assert!(replayed.is_ready_to_complete().unwrap());
        replayed.complete().unwrap();
        assert!(matches!(
            replayed.uncommitted_events().last().unwrap().payload(),
            TeamMembershipCleanupEventPayload::Completed {
                succeeded_count, failed_count
            } if succeeded_count.value() == 1 && failed_count.value() == 1
        ));
    }
}
