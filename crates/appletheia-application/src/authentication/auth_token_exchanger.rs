use crate::unit_of_work::UnitOfWork;

use super::{AuthTokenExchangeRequest, AuthTokenExchangeResult, AuthTokenExchangerError};

/// Exchanges one-time codes for newly issued auth tokens.
#[allow(async_fn_in_trait)]
pub trait AuthTokenExchanger: Send + Sync {
    /// The unit of work type used by the exchanger.
    type Uow: UnitOfWork;

    /// Exchanges the provided request for tokens.
    async fn exchange(
        &self,
        uow: &mut Self::Uow,
        request: AuthTokenExchangeRequest,
    ) -> Result<AuthTokenExchangeResult, AuthTokenExchangerError>;
}
