use super::{MintAccountCreateReceipt, MintAccountCreateRequest, MintAccountCreatorError};

/// Creates or retrieves an on-chain mint account.
#[allow(async_fn_in_trait)]
pub trait MintAccountCreator: Send + Sync {
    async fn create_or_get(
        &self,
        request: MintAccountCreateRequest,
    ) -> Result<MintAccountCreateReceipt, MintAccountCreatorError>;
}
