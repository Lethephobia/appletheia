use appletheia::application::read_model::{ReadModel, ReadModelName};
use serde::Serialize;

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

impl ReadModel for PublicOrganizationList {
    const NAME: ReadModelName = ReadModelName::new("public_organization_list");
}
