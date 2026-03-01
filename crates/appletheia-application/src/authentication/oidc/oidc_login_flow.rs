use super::{
    OidcAuthorizationUrl, OidcBeginOptions, OidcCallbackParams, OidcCompleteResult,
    OidcLoginFlowError,
};

#[allow(async_fn_in_trait)]
pub trait OidcLoginFlow: Send + Sync {
    async fn begin(
        &self,
        options: OidcBeginOptions,
    ) -> Result<OidcAuthorizationUrl, OidcLoginFlowError>;

    async fn complete(
        &self,
        callback_params: OidcCallbackParams,
    ) -> Result<OidcCompleteResult, OidcLoginFlowError>;
}
