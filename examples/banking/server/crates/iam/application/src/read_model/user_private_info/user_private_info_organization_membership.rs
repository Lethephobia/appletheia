use appletheia::domain::EventId;
use banking_iam_domain::OrganizationRoles;

use crate::read_model::ReadModelObservation;

use super::UserPrivateInfoOrganization;

/// Organization membership shown in user-private information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserPrivateInfoOrganizationMembership {
    pub organization: UserPrivateInfoOrganization,
    pub roles: OrganizationRoles,
    pub observation: ReadModelObservation,
}

impl UserPrivateInfoOrganizationMembership {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        ReadModelObservation::collect_event_ids(
            self.observation
                .event_ids()
                .chain(self.organization.observation.event_ids()),
        )
    }
}
