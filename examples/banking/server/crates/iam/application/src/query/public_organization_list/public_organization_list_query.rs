use appletheia::query;
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

use crate::read_model::{
    PublicOrganizationListCriteria, PublicOrganizationListCursor, PublicOrganizationListSortKey,
};

/// Query parameters for public organization list reads.
#[query(name = "public_organization_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicOrganizationListQuery {
    pub criteria: PublicOrganizationListCriteria,
    pub cursor_options:
        Option<CursorOptions<PublicOrganizationListSortKey, PublicOrganizationListCursor>>,
    pub limit: PageSize,
}
