use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UnitOfWorkFactoryError {
    #[error("begin failed {0}")]
    BeginFailed(#[source] Box<dyn Error + Send + Sync + 'static>),
}
