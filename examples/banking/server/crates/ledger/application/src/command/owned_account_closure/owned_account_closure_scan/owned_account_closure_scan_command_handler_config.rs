use appletheia::application::repository::ReferenceIndexLookupPageSize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnedAccountClosureScanCommandHandlerConfig {
    pub page_size: ReferenceIndexLookupPageSize,
}
