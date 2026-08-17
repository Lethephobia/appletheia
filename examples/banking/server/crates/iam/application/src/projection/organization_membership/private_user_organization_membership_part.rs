use appletheia::application::read_model::{
    ReadModelFragment, ReadModelObservation, ReadModelObservationSource, ReadModelPart,
    ReadModelPartName, ReadModelPartTree,
};
use banking_iam_domain::{OrganizationRoles, UserId};
use serde::{Deserialize, Serialize};

use crate::projection::InternalOrganizationSummaryPart;

use super::{OrganizationMembershipFragment, OrganizationMembershipFragmentKey};

/// Private organization-membership relationship visible only to the owning user.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PrivateUserOrganizationMembershipPart {
    pub user_id: UserId,
    pub organization: InternalOrganizationSummaryPart,
    pub roles: OrganizationRoles,
    pub observation: ReadModelObservation,
}

impl From<OrganizationMembershipFragment> for PrivateUserOrganizationMembershipPart {
    fn from(fragment: OrganizationMembershipFragment) -> Self {
        Self {
            user_id: fragment.user.id,
            organization: fragment.organization.into(),
            roles: fragment.roles,
            observation: fragment.observation,
        }
    }
}

impl ReadModelObservationSource for PrivateUserOrganizationMembershipPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation, self.organization.observation]
    }
}

impl ReadModelPart for PrivateUserOrganizationMembershipPart {
    const NAME: ReadModelPartName = ReadModelPartName::new("private_user_organization_membership");

    type SourceFragment = OrganizationMembershipFragment;

    fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
        OrganizationMembershipFragmentKey {
            user_id: self.user_id,
            organization_id: self.organization.organization_id,
        }
    }

    fn parts(part: Option<&Self>) -> Vec<ReadModelPartTree> {
        vec![ReadModelPartTree::field::<InternalOrganizationSummaryPart>(
            "organization",
            part.map(|membership| &membership.organization),
        )]
    }
}
