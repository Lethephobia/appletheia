use super::{ReadModelIncludedError, ReadModelRef, ReadModelResource};

/// Converts one variant of an application's heterogeneous included resource views.
/// Each variant can hold a ReadModelView with its own field filter.
pub trait ReadModelIncluded: Send + Sync {
    fn read_model_ref(&self) -> ReadModelRef;

    fn try_to_resource(&self) -> Result<ReadModelResource, ReadModelIncludedError>;
}
