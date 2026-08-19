use appletheia::application::read_model::{
    ReadModelFragment, ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{OrganizationId, OrganizationRoles, UserId};
use serde::{Deserialize, Serialize};

use super::OrganizationMembershipFragmentKey;

/// Normalized organization membership fragment shared by read models.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrganizationMembershipFragment {
    pub user_id: UserId,
    pub organization_id: OrganizationId,
    pub roles: OrganizationRoles,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl ReadModelObservationSource for OrganizationMembershipFragment {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation]
    }
}

impl ReadModelFragment for OrganizationMembershipFragment {
    const NAME: ReadModelFragmentName =
        ReadModelFragmentName::new("organization_membership_fragment");

    type Key = OrganizationMembershipFragmentKey;

    fn key(&self) -> Self::Key {
        OrganizationMembershipFragmentKey {
            user_id: self.user_id,
            organization_id: self.organization_id,
        }
    }
}
