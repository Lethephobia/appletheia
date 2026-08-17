use appletheia::application::read_model::{
    ReadModelFragment, ReadModelObservation, ReadModelObservationSource, ReadModelPart,
    ReadModelPartName,
};
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};
use serde::{Deserialize, Serialize};

use super::OrganizationFragment;

/// Basic organization summary shared by authenticated IAM read models.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InternalOrganizationSummaryPart {
    pub organization_id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
    pub observation: ReadModelObservation,
}

impl From<OrganizationFragment> for InternalOrganizationSummaryPart {
    fn from(fragment: OrganizationFragment) -> Self {
        Self {
            organization_id: fragment.id,
            handle: fragment.handle,
            display_name: fragment.display_name,
            picture: fragment.picture,
            observation: fragment.observation,
        }
    }
}

impl ReadModelObservationSource for InternalOrganizationSummaryPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation]
    }
}

impl ReadModelPart for InternalOrganizationSummaryPart {
    const NAME: ReadModelPartName = ReadModelPartName::new("internal_organization_summary");

    type SourceFragment = OrganizationFragment;

    fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
        self.organization_id
    }
}
