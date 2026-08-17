use appletheia::application::read_model::{
    ReadModelFragment, ReadModelObservation, ReadModelObservationSource, ReadModelPart,
    ReadModelPartName,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationPictureRef, OrganizationWebsiteUrl,
};
use serde::{Deserialize, Serialize};

use super::OrganizationFragment;

/// Detailed organization data visible to organization members.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InternalOrganizationDetailsPart {
    pub organization_id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
    pub observation: ReadModelObservation,
    pub description: Option<OrganizationDescription>,
    pub website_url: Option<OrganizationWebsiteUrl>,
    pub created_at: EventOccurredAt,
}

impl From<OrganizationFragment> for InternalOrganizationDetailsPart {
    fn from(fragment: OrganizationFragment) -> Self {
        Self {
            organization_id: fragment.id,
            handle: fragment.handle,
            display_name: fragment.display_name,
            picture: fragment.picture,
            observation: fragment.observation,
            description: fragment.description,
            website_url: fragment.website_url,
            created_at: fragment.created_at,
        }
    }
}

impl ReadModelObservationSource for InternalOrganizationDetailsPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation]
    }
}

impl ReadModelPart for InternalOrganizationDetailsPart {
    const NAME: ReadModelPartName = ReadModelPartName::new("internal_organization_details");

    type SourceFragment = OrganizationFragment;

    fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
        self.organization_id
    }
}
