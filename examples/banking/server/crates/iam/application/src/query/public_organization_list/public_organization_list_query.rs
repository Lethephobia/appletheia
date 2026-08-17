use appletheia::application::read_model::pagination::{CursorPage, Sort};
use appletheia::query;

use crate::read_model::{
    PublicOrganizationListCriteria, PublicOrganizationListCursor, PublicOrganizationListSortKey,
};

/// Query parameters for public organization list reads.
#[query(name = "public_organization_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicOrganizationListQuery {
    pub criteria: PublicOrganizationListCriteria,
    pub sort: Sort<PublicOrganizationListSortKey>,
    pub page: CursorPage<PublicOrganizationListCursor>,
}
