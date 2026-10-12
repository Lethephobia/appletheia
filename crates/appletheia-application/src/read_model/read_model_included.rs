use super::{ReadModelIncludedError, ReadModelResource};

/// Converts one variant of an application's heterogeneous included resource views.
/// Each variant can hold a ReadModelView with its own field filter.
pub trait ReadModelIncluded: Send + Sync {
    fn try_to_resource(&self) -> Result<ReadModelResource, ReadModelIncludedError>;
}
