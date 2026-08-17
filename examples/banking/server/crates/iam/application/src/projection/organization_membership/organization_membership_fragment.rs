use appletheia::application::read_model::{
    ReadModelFragment, ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::OrganizationRoles;
use serde::{Deserialize, Serialize};

use super::OrganizationMembershipFragmentKey;
use super::{OrganizationFragment, UserFragment};

/// Complete organization membership fragment shared by read models.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrganizationMembershipFragment {
    pub user: UserFragment,
    pub organization: OrganizationFragment,
    pub roles: OrganizationRoles,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl ReadModelObservationSource for OrganizationMembershipFragment {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.user
            .observations()
            .into_iter()
            .chain(self.organization.observations())
            .chain([self.observation])
            .collect()
    }
}

impl ReadModelFragment for OrganizationMembershipFragment {
    const NAME: ReadModelFragmentName =
        ReadModelFragmentName::new("organization_membership_fragment");

    type Key = OrganizationMembershipFragmentKey;

    fn key(&self) -> Self::Key {
        OrganizationMembershipFragmentKey {
            user_id: self.user.id,
            organization_id: self.organization.id,
        }
    }
}
