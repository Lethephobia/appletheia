use appletheia::application::read_model::{
    ReadModelFragment, ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationPictureRef, OrganizationWebsiteUrl, UserId,
};
use serde::{Deserialize, Serialize};

/// Normalized organization fragment stored once and shared by read models.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrganizationFragment {
    pub id: OrganizationId,
    pub owner_user_id: UserId,
    pub owner_since: EventOccurredAt,
    pub owner_observation: ReadModelObservation,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub description: Option<OrganizationDescription>,
    pub website_url: Option<OrganizationWebsiteUrl>,
    pub picture: Option<OrganizationPictureRef>,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl ReadModelObservationSource for OrganizationFragment {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.owner_observation, self.observation]
    }
}

impl ReadModelFragment for OrganizationFragment {
    const NAME: ReadModelFragmentName = ReadModelFragmentName::new("organization_fragment");

    type Key = OrganizationId;

    fn key(&self) -> Self::Key {
        self.id
    }
}
