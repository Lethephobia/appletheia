use std::error::Error;

use thiserror::Error;

use crate::unit_of_work::UnitOfWorkError;

#[derive(Debug, Error)]
pub enum CommandDispatchError<HE>
where
    HE: Error + Send + Sync + 'static,
{
    #[error("unit of work error: {0}")]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error("command handler error: {0}")]
    Handler(#[source] HE),
}
