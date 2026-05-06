use std::error::Error as StdError;

use thiserror::Error;

use crate::authorization::AuthorizerError;
use crate::projection::ReadYourWritesWaitError;
use crate::unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError};

#[derive(Debug, Error)]
pub enum QueryDispatcherError<HE>
where
    HE: StdError + Send + Sync + 'static,
{
    #[error("unit of work factory error: {0}")]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error("unit of work error: {0}")]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error(transparent)]
    ReadYourWrites(#[from] ReadYourWritesWaitError),

    #[error("query handler error: {0}")]
    Handler(#[source] HE),

    #[error("authorizer error: {0}")]
    Authorizer(#[from] AuthorizerError),
}
