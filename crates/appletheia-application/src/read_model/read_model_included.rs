use super::{ReadModelIncludedError, ReadModelResource};

/// Converts one variant of an application's heterogeneous included resources.
pub trait ReadModelIncluded: Send + Sync {
    fn try_to_resource(&self) -> Result<ReadModelResource, ReadModelIncludedError>;
}
