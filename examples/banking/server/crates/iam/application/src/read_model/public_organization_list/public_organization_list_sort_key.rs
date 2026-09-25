/// Sort key for public organization list reads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PublicOrganizationListSortKey {
    CreatedAt,
    OrganizationId,
}
