use thiserror::Error;

use crate::Retryability;
use crate::authorization::AuthorizerError;
use crate::unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError};

#[derive(Debug, Error)]
pub enum QueryDispatcherError<HE>
where
    HE: Retryability,
{
    #[error("unit of work factory error: {0}")]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error("unit of work error: {0}")]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error("query handler error: {0}")]
    Handler(#[source] HE),

    #[error("authorizer error: {0}")]
    Authorizer(#[from] AuthorizerError),
}

impl<HE> Retryability for QueryDispatcherError<HE>
where
    HE: Retryability,
{
    fn is_retryable(&self) -> bool {
        match self {
            Self::Handler(error) => error.is_retryable(),
            Self::Authorizer(
                AuthorizerError::PrincipalUnavailable | AuthorizerError::Forbidden,
            ) => false,
            Self::UnitOfWorkFactory(_)
            | Self::UnitOfWork(_)
            | Self::Authorizer(AuthorizerError::Backend(_)) => true,
        }
    }
}
