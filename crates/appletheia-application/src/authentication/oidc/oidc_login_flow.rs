use crate::unit_of_work::UnitOfWork;

use super::{
    OidcBeginOptions, OidcBeginResult, OidcCallbackParams, OidcCompleteResult, OidcLoginFlowError,
};

#[allow(async_fn_in_trait)]
pub trait OidcLoginFlow: Send + Sync {
    type Uow: UnitOfWork;

    async fn begin(
        &self,
        uow: &mut Self::Uow,
        options: OidcBeginOptions,
    ) -> Result<OidcBeginResult, OidcLoginFlowError>;

    async fn complete(
        &self,
        uow: &mut Self::Uow,
        callback_params: OidcCallbackParams,
    ) -> Result<OidcCompleteResult, OidcLoginFlowError>;
}
