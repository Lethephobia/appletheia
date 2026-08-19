use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    SerializedPartition, SerializedPartitionError,
};
use serde::Serialize;

use crate::projection::OrganizationFragment;

mod public_organization_list_criteria;
mod public_organization_list_cursor;
mod public_organization_list_item;
mod public_organization_list_reader;
mod public_organization_list_reader_error;
mod public_organization_list_sort_key;

pub use public_organization_list_criteria::PublicOrganizationListCriteria;
pub use public_organization_list_cursor::PublicOrganizationListCursor;
pub use public_organization_list_item::PublicOrganizationListItem;
pub use public_organization_list_reader::PublicOrganizationListReader;
pub use public_organization_list_reader_error::PublicOrganizationListReaderError;
pub use public_organization_list_sort_key::PublicOrganizationListSortKey;

/// Read model for public organization list reads.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PublicOrganizationList {
    pub items: Vec<PublicOrganizationListItem>,
    pub start_cursor: Option<PublicOrganizationListCursor>,
    pub end_cursor: Option<PublicOrganizationListCursor>,
    pub has_previous: bool,
    pub has_next: bool,
}

impl ReadModelObservationSource for PublicOrganizationList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.items.iter().map(|item| item.observation).collect()
    }
}

impl ReadModel for PublicOrganizationList {
    const NAME: ReadModelName = ReadModelName::new("public_organization_list");

    fn partitions(&self) -> Result<Vec<SerializedPartition>, SerializedPartitionError> {
        self.items
            .iter()
            .map(|item| {
                SerializedPartition::try_from_fragment_key::<OrganizationFragment>(
                    &item.organization_id,
                )
            })
            .collect()
    }
}
