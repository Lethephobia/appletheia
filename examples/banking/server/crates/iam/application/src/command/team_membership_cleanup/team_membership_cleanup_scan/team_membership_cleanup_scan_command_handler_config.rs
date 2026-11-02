use appletheia::application::repository::ReferenceIndexLookupPageSize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TeamMembershipCleanupScanCommandHandlerConfig {
    pub page_size: ReferenceIndexLookupPageSize,
}
