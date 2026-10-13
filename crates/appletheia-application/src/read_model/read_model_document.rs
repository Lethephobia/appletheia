use super::{ReadModelDocumentError, ReadModelRef, ReadModelResourceDocument};

pub trait ReadModelDocument: Send + Sync {
    /// Collects distinct primary and included resource references without serializing fields.
    fn read_model_refs(&self) -> Vec<ReadModelRef>;

    fn try_to_resource_document(&self)
    -> Result<ReadModelResourceDocument, ReadModelDocumentError>;
}
