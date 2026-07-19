mod public_organization_list_criteria;
mod public_organization_list_cursor;
mod public_organization_list_item;
mod public_organization_list_reader;
mod public_organization_list_reader_error;
mod public_organization_list_sort_key;
mod public_organization_list_upsert;
mod public_organization_list_writer;
mod public_organization_list_writer_error;

pub use public_organization_list_criteria::PublicOrganizationListCriteria;
pub use public_organization_list_cursor::PublicOrganizationListCursor;
pub use public_organization_list_item::PublicOrganizationListItem;
pub use public_organization_list_reader::PublicOrganizationListReader;
pub use public_organization_list_reader_error::PublicOrganizationListReaderError;
pub use public_organization_list_sort_key::PublicOrganizationListSortKey;
pub use public_organization_list_upsert::PublicOrganizationListUpsert;
pub use public_organization_list_writer::PublicOrganizationListWriter;
pub use public_organization_list_writer_error::PublicOrganizationListWriterError;

/// Read model for public organization list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicOrganizationList {
    pub items: Vec<PublicOrganizationListItem>,
    pub next_cursor: Option<PublicOrganizationListCursor>,
}
