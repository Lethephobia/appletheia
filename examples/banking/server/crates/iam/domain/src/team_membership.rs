mod team_membership_error;
mod team_membership_event_payload;
mod team_membership_event_payload_error;
mod team_membership_id;
mod team_membership_state;
mod team_membership_state_error;
mod team_membership_status;

pub use team_membership_error::TeamMembershipError;
pub use team_membership_event_payload::TeamMembershipEventPayload;
pub use team_membership_event_payload_error::TeamMembershipEventPayloadError;
pub use team_membership_id::TeamMembershipId;
pub use team_membership_state::TeamMembershipState;
pub use team_membership_state_error::TeamMembershipStateError;
pub use team_membership_status::TeamMembershipStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::{OrganizationMembershipId, TeamId};

/// Binds Team membership to one OrganizationMembership joining lifecycle.
/// Removal is terminal; rejoining creates a new TeamMembership.
#[aggregate(type = "team_membership", error = TeamMembershipError)]
pub struct TeamMembership {
    core: AggregateCore<TeamMembershipId, TeamMembershipState, TeamMembershipEventPayload>,
}

impl TeamMembership {
    pub fn team_id(&self) -> Result<TeamId, TeamMembershipError> {
        Ok(self.state_required()?.team_id)
    }

    pub fn organization_membership_id(
        &self,
    ) -> Result<OrganizationMembershipId, TeamMembershipError> {
        Ok(self.state_required()?.organization_membership_id)
    }

    pub fn status(&self) -> Result<TeamMembershipStatus, TeamMembershipError> {
        Ok(self.state_required()?.status)
    }

    pub fn is_active(&self) -> Result<bool, TeamMembershipError> {
        Ok(self.state_required()?.status.is_active())
    }

    pub fn is_removed(&self) -> Result<bool, TeamMembershipError> {
        Ok(self.state_required()?.status.is_removed())
    }

    pub fn create(
        &mut self,
        team_id: TeamId,
        organization_membership_id: OrganizationMembershipId,
    ) -> Result<(), TeamMembershipError> {
        if self.state().is_some() {
            return Err(TeamMembershipError::AlreadyCreated);
        }

        self.append_event(TeamMembershipEventPayload::Created {
            team_id,
            organization_membership_id,
        })?;
        Ok(())
    }

    pub fn remove(&mut self) -> Result<(), TeamMembershipError> {
        if self.state_required()?.status.is_removed() {
            return Err(TeamMembershipError::Removed);
        }

        let state = self.state_required()?;
        let team_id = state.team_id;
        let organization_membership_id = state.organization_membership_id;
        self.append_event(TeamMembershipEventPayload::Removed {
            team_id,
            organization_membership_id,
        })?;
        Ok(())
    }
}

impl AggregateApply<TeamMembershipEventPayload, TeamMembershipError> for TeamMembership {
    fn apply(&mut self, payload: &TeamMembershipEventPayload) -> Result<(), TeamMembershipError> {
        match payload {
            TeamMembershipEventPayload::Created {
                team_id,
                organization_membership_id,
            } => {
                self.set_state(Some(TeamMembershipState {
                    team_id: *team_id,
                    organization_membership_id: *organization_membership_id,
                    status: TeamMembershipStatus::Active,
                }));
            }
            TeamMembershipEventPayload::Removed { .. } => {
                self.state_required_mut()?.status = TeamMembershipStatus::Removed;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, AggregateId, EventPayload};

    use super::{
        TeamMembership, TeamMembershipError, TeamMembershipEventPayload, TeamMembershipStatus,
    };
    use crate::{OrganizationMembershipId, TeamId};

    fn created_membership() -> TeamMembership {
        let mut membership = TeamMembership::new();
        membership
            .create(TeamId::new(), OrganizationMembershipId::new())
            .expect("create should succeed");
        membership
    }

    #[test]
    fn create_initializes_state_and_records_event() {
        let team_id = TeamId::new();
        let organization_membership_id = OrganizationMembershipId::new();
        let mut membership = TeamMembership::new();

        membership
            .create(team_id, organization_membership_id)
            .expect("create should succeed");

        assert!(!membership.aggregate_id().value().is_nil());
        assert_eq!(membership.team_id().expect("team id should exist"), team_id);
        assert_eq!(
            membership
                .organization_membership_id()
                .expect("organization membership id should exist"),
            organization_membership_id
        );
        assert_eq!(
            membership.status().expect("status should exist"),
            TeamMembershipStatus::Active
        );
        assert_eq!(membership.uncommitted_events().len(), 1);
        assert_eq!(
            membership.uncommitted_events()[0].payload().name(),
            TeamMembershipEventPayload::CREATED
        );
    }

    #[test]
    fn creating_twice_fails() {
        let mut membership = created_membership();

        let error = membership
            .create(TeamId::new(), OrganizationMembershipId::new())
            .expect_err("second create should fail");

        assert!(matches!(error, super::TeamMembershipError::AlreadyCreated));
    }

    #[test]
    fn removing_membership_updates_status_and_records_event() {
        let mut membership = created_membership();

        membership.remove().expect("remove should succeed");

        assert_eq!(
            membership.status().expect("status should exist"),
            TeamMembershipStatus::Removed
        );
        assert_eq!(membership.uncommitted_events().len(), 2);
        assert_eq!(
            membership.uncommitted_events()[1].payload().name(),
            TeamMembershipEventPayload::REMOVED
        );
    }

    #[test]
    fn removing_twice_is_rejected() {
        let mut membership = created_membership();
        membership.remove().expect("remove should succeed");

        let error = membership.remove().expect_err("second remove should fail");

        assert!(matches!(error, TeamMembershipError::Removed));
        assert_eq!(membership.uncommitted_events().len(), 2);
    }

    #[test]
    fn membership_indexes_follow_the_joining_lifecycle() {
        use super::TeamMembershipState;
        use appletheia::domain::{AggregateApply, UniqueValue};

        let team_id = TeamId::new();
        let organization_membership_id = OrganizationMembershipId::new();
        let mut membership = TeamMembership::new();
        membership
            .create(team_id, organization_membership_id)
            .unwrap();
        let expected_unique_value = UniqueValue::from_strings([
            team_id.value().to_string().as_str(),
            organization_membership_id.value().to_string().as_str(),
        ])
        .unwrap();
        assert_eq!(
            membership
                .unique_entries()
                .unwrap()
                .get(TeamMembershipState::TEAM_ORGANIZATION_MEMBERSHIP_KEY)
                .unwrap()
                .first(),
            Some(&expected_unique_value)
        );
        let references = membership.reference_entries().unwrap();
        assert_eq!(
            references
                .get(TeamMembershipState::TEAM_REF)
                .unwrap()
                .iter()
                .collect::<Vec<_>>(),
            vec![team_id.value()]
        );
        assert_eq!(
            references
                .get(TeamMembershipState::ORGANIZATION_MEMBERSHIP_REF)
                .unwrap()
                .iter()
                .collect::<Vec<_>>(),
            vec![organization_membership_id.value()]
        );

        let mut later_membership = TeamMembership::new();
        later_membership
            .create(team_id, OrganizationMembershipId::new())
            .unwrap();
        assert_ne!(
            later_membership.unique_entries().unwrap(),
            membership.unique_entries().unwrap()
        );

        membership.remove().unwrap();
        assert!(membership.unique_entries().unwrap().is_empty());
        assert_eq!(membership.reference_entries().unwrap(), references);
        let mut replayed = TeamMembership::new();
        for event in membership.uncommitted_events() {
            replayed.apply(event.payload()).unwrap();
        }
        assert_eq!(replayed.state(), membership.state());
        assert!(replayed.unique_entries().unwrap().is_empty());
        assert_eq!(replayed.reference_entries().unwrap(), references);
    }
}
