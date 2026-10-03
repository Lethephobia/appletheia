use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelRelationshipValueError {
    #[error("relationship linkage conversion failed")]
    Conversion(#[source] Box<dyn Error + Send + Sync>),
}
