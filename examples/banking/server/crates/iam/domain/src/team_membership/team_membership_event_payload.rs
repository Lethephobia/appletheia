use appletheia::event_payload;

use crate::{OrganizationMembershipId, TeamId};

use super::TeamMembershipEventPayloadError;

#[event_payload(error = TeamMembershipEventPayloadError)]
pub enum TeamMembershipEventPayload {
    Created {
        team_id: TeamId,
        organization_membership_id: OrganizationMembershipId,
    },
    PromotedToManager,
    DemotedFromManager,
    Removed {
        team_id: TeamId,
        organization_membership_id: OrganizationMembershipId,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use super::TeamMembershipEventPayload;
    use crate::{OrganizationMembershipId, TeamId};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            TeamMembershipEventPayload::CREATED,
            appletheia::domain::EventName::new("created")
        );
        assert_eq!(
            TeamMembershipEventPayload::REMOVED,
            appletheia::domain::EventName::new("removed")
        );
    }

    #[test]
    fn created_payload_name_matches_variant() {
        let payload = TeamMembershipEventPayload::Created {
            team_id: TeamId::new(),
            organization_membership_id: OrganizationMembershipId::new(),
        };

        assert_eq!(payload.name(), TeamMembershipEventPayload::CREATED);
    }

    #[test]
    fn removed_payload_name_matches_variant() {
        let payload = TeamMembershipEventPayload::Removed {
            team_id: TeamId::new(),
            organization_membership_id: OrganizationMembershipId::new(),
        };

        assert_eq!(payload.name(), TeamMembershipEventPayload::REMOVED);
    }

    #[test]
    fn serializes_payload_to_json() {
        let payload = TeamMembershipEventPayload::Created {
            team_id: TeamId::new(),
            organization_membership_id: OrganizationMembershipId::new(),
        };

        let value = payload
            .try_into_json_value()
            .expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("created"));
    }
}
