use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource, ReadModelPartTree,
};
use serde::{Deserialize, Serialize};

use crate::projection::{InternalOrganizationDetailsPart, OrganizationFragment};

mod organization_internal_info_reader;
mod organization_internal_info_reader_error;

pub use organization_internal_info_reader::OrganizationInternalInfoReader;
pub use organization_internal_info_reader_error::OrganizationInternalInfoReaderError;

/// Organization information visible to its members.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrganizationInternalInfo {
    pub organization: InternalOrganizationDetailsPart,
}

impl From<OrganizationFragment> for OrganizationInternalInfo {
    fn from(fragment: OrganizationFragment) -> Self {
        Self {
            organization: fragment.into(),
        }
    }
}

impl ReadModelObservationSource for OrganizationInternalInfo {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.organization.observations()
    }
}

impl ReadModel for OrganizationInternalInfo {
    const NAME: ReadModelName = ReadModelName::new("organization_internal_info");

    fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree> {
        vec![ReadModelPartTree::field::<InternalOrganizationDetailsPart>(
            "organization",
            read_model.map(|read_model| &read_model.organization),
        )]
    }
}
