use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource, ReadModelPartTree,
};

use crate::projection::PublicOrganizationListItemPart;

mod public_organization_list_criteria;
mod public_organization_list_cursor;
mod public_organization_list_reader;
mod public_organization_list_reader_error;
mod public_organization_list_sort_key;

pub use public_organization_list_criteria::PublicOrganizationListCriteria;
pub use public_organization_list_cursor::PublicOrganizationListCursor;
pub use public_organization_list_reader::PublicOrganizationListReader;
pub use public_organization_list_reader_error::PublicOrganizationListReaderError;
pub use public_organization_list_sort_key::PublicOrganizationListSortKey;

/// Read model for public organization list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicOrganizationList {
    pub items: Vec<PublicOrganizationListItemPart>,
    pub next_cursor: Option<PublicOrganizationListCursor>,
}

impl ReadModelObservationSource for PublicOrganizationList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.items
            .iter()
            .flat_map(ReadModelObservationSource::observations)
            .collect()
    }
}

impl ReadModel for PublicOrganizationList {
    const NAME: ReadModelName = ReadModelName::new("public_organization_list");

    fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree> {
        vec![ReadModelPartTree::collection::<
            PublicOrganizationListItemPart,
        >(
            "items",
            read_model.map(|read_model| read_model.items.as_slice()),
        )]
    }
}
