use thiserror::Error;

use crate::read_model::{ReadModelPartChangeEnvelopeError, ReadModelPartChangeError};

/// Reports a failure while mapping and filtering one fragment-change delivery.
#[derive(Debug, Error)]
pub enum DefaultReadModelWatchChangeRouterError {
    #[error("failed to map a fragment change through the read model part tree")]
    PartChange(#[from] ReadModelPartChangeError),

    #[error("failed to build a read model part change envelope")]
    PartChangeEnvelope(#[from] ReadModelPartChangeEnvelopeError),
}
