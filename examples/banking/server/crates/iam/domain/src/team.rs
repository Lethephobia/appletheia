mod team_description;
mod team_description_error;
mod team_error;
mod team_event_payload;
mod team_event_payload_error;
mod team_id;
mod team_name;
mod team_name_error;
mod team_state;
mod team_state_error;
mod team_status;

pub use team_description::TeamDescription;
pub use team_description_error::TeamDescriptionError;
pub use team_error::TeamError;
pub use team_event_payload::TeamEventPayload;
pub use team_event_payload_error::TeamEventPayloadError;
pub use team_id::TeamId;
pub use team_name::TeamName;
pub use team_name_error::TeamNameError;
pub use team_state::TeamState;
pub use team_state_error::TeamStateError;
pub use team_status::TeamStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::OrganizationId;

/// An organization-scoped group whose membership is managed by separate aggregates.
#[aggregate(type = "team", error = TeamError)]
pub struct Team {
    core: AggregateCore<TeamId, TeamState, TeamEventPayload>,
}

impl Team {
    pub fn organization_id(&self) -> Result<OrganizationId, TeamError> {
        Ok(self.state_required()?.organization_id)
    }

    pub fn name(&self) -> Result<&TeamName, TeamError> {
        Ok(&self.state_required()?.name)
    }

    pub fn description(&self) -> Result<Option<&TeamDescription>, TeamError> {
        Ok(self.state_required()?.description.as_ref())
    }

    pub fn status(&self) -> Result<TeamStatus, TeamError> {
        Ok(self.state_required()?.status)
    }

    pub fn is_active(&self) -> Result<bool, TeamError> {
        Ok(self.state_required()?.status.is_active())
    }

    pub fn is_deleted(&self) -> Result<bool, TeamError> {
        Ok(self.state_required()?.status.is_deleted())
    }

    pub fn create(
        &mut self,
        organization_id: OrganizationId,
        name: TeamName,
    ) -> Result<(), TeamError> {
        if self.state().is_some() {
            return Err(TeamError::AlreadyCreated);
        }

        self.append_event(TeamEventPayload::Created {
            organization_id,
            name,
        })?;
        Ok(())
    }

    pub fn change_name(&mut self, name: TeamName) -> Result<(), TeamError> {
        if self.state_required()?.status.is_deleted() {
            return Err(TeamError::Deleted);
        }

        self.append_event(TeamEventPayload::NameChanged { name })?;
        Ok(())
    }

    pub fn set_description(
        &mut self,
        description: Option<TeamDescription>,
    ) -> Result<(), TeamError> {
        if self.state_required()?.status.is_deleted() {
            return Err(TeamError::Deleted);
        }

        self.append_event(TeamEventPayload::DescriptionSet { description })?;
        Ok(())
    }

    pub fn delete(&mut self) -> Result<(), TeamError> {
        if self.state_required()?.status.is_deleted() {
            return Err(TeamError::Deleted);
        }

        self.append_event(TeamEventPayload::Deleted)?;
        Ok(())
    }
}

impl AggregateApply<TeamEventPayload, TeamError> for Team {
    fn apply(&mut self, payload: &TeamEventPayload) -> Result<(), TeamError> {
        match payload {
            TeamEventPayload::Created {
                organization_id,
                name,
            } => self.set_state(Some(TeamState {
                organization_id: *organization_id,
                name: name.clone(),
                description: None,
                status: TeamStatus::Active,
            })),
            TeamEventPayload::NameChanged { name } => {
                self.state_required_mut()?.name = name.clone();
            }
            TeamEventPayload::DescriptionSet { description } => {
                self.state_required_mut()?.description = description.clone();
            }
            TeamEventPayload::Deleted => {
                self.state_required_mut()?.status = TeamStatus::Deleted;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, AggregateApply, AggregateId, EventPayload};

    use super::{Team, TeamDescription, TeamError, TeamEventPayload, TeamName, TeamState};
    use crate::OrganizationId;

    #[test]
    fn events_reconstruct_team_lifecycle_and_organization_reference() {
        let organization_id = OrganizationId::new();
        let mut team = Team::new();
        team.create(organization_id, TeamName::try_from("Finance").unwrap())
            .unwrap();
        assert!(team.is_active().unwrap());
        assert!(team.description().unwrap().is_none());
        assert_eq!(team.organization_id().unwrap(), organization_id);
        assert_eq!(team.uncommitted_events().len(), 1);
        team.change_name(TeamName::try_from("Treasury").unwrap())
            .unwrap();
        assert_eq!(team.name().unwrap().value(), "Treasury");
        team.set_description(Some(
            TeamDescription::try_from("Treasury operations").unwrap(),
        ))
        .unwrap();
        assert_eq!(
            team.description().unwrap().unwrap().value(),
            "Treasury operations"
        );
        team.set_description(None).unwrap();
        assert!(team.description().unwrap().is_none());
        team.delete().unwrap();
        assert!(team.is_deleted().unwrap());

        let mut replayed = Team::new();
        for event in team.uncommitted_events() {
            let encoded = serde_json::to_value(event.payload()).unwrap();
            let decoded: TeamEventPayload = serde_json::from_value(encoded).unwrap();
            replayed.apply(&decoded).unwrap();
        }
        assert_eq!(replayed.state(), team.state());
        assert_eq!(
            team.uncommitted_events()
                .iter()
                .map(|event| event.payload().name())
                .collect::<Vec<_>>(),
            vec![
                TeamEventPayload::CREATED,
                TeamEventPayload::NAME_CHANGED,
                TeamEventPayload::DESCRIPTION_SET,
                TeamEventPayload::DESCRIPTION_SET,
                TeamEventPayload::DELETED,
            ]
        );
        assert_eq!(
            replayed
                .reference_entries()
                .unwrap()
                .get(TeamState::ORGANIZATION_REF)
                .unwrap()
                .iter()
                .collect::<Vec<_>>(),
            vec![organization_id.value()]
        );
    }

    #[test]
    fn duplicate_creation_and_operations_after_deletion_do_not_append_events() {
        let mut team = Team::new();
        let organization_id = OrganizationId::new();
        let name = TeamName::try_from("Finance").unwrap();
        team.create(organization_id, name.clone()).unwrap();
        assert!(matches!(
            team.create(organization_id, name.clone()),
            Err(TeamError::AlreadyCreated)
        ));
        assert_eq!(team.uncommitted_events().len(), 1);
        team.delete().unwrap();
        for result in [
            team.change_name(name.clone()),
            team.set_description(Some(TeamDescription::try_from("Updated").unwrap())),
            team.set_description(None),
            team.delete(),
        ] {
            assert!(matches!(result, Err(TeamError::Deleted)));
        }
        assert!(matches!(
            team.create(organization_id, name),
            Err(TeamError::AlreadyCreated)
        ));
        assert_eq!(team.uncommitted_events().len(), 2);
        assert!(team.is_deleted().unwrap());
    }

    #[test]
    fn accepted_unchanged_values_still_append_events() {
        let mut team = Team::new();
        let name = TeamName::try_from("Finance").unwrap();
        team.create(OrganizationId::new(), name.clone()).unwrap();
        team.change_name(name).unwrap();
        team.set_description(None).unwrap();
        let description = TeamDescription::try_from("Finance operations").unwrap();
        team.set_description(Some(description.clone())).unwrap();
        team.set_description(Some(description)).unwrap();
        assert_eq!(team.uncommitted_events().len(), 5);
    }

    #[test]
    fn operations_before_creation_do_not_initialize_state_or_append_events() {
        let mut team = Team::new();
        for result in [
            team.change_name(TeamName::try_from("Finance").unwrap()),
            team.set_description(None),
            team.delete(),
        ] {
            assert!(matches!(result, Err(TeamError::Aggregate(_))));
        }
        assert!(team.state().is_none());
        assert!(team.uncommitted_events().is_empty());
    }
}
